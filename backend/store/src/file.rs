use std::collections::HashMap;

pub struct FileStore {
    filepath: String,
}

impl FileStore {
    pub fn new(filepath: String) -> Self {
        FileStore { filepath }
    }

    pub fn get_data(&self, key: &String) -> Option<String> {
        let mut reader = self.get_reader();
        let mut ret_val = None;
        for record in reader.records() {
            tracing::debug!("{:?}", record);
            let r = record.unwrap();
            if &r[0] == key {
                ret_val = Some(String::from(&r[1]));
                break;
            }
        }
        ret_val
    }

    pub fn put_data(&mut self, key: &String, value: &String) {
        let mut writer = self.get_writer(true);
        writer.write_record(&[key, value]).ok();
        writer.flush().ok();
    }

    pub fn remove_data(&mut self, key: &String) -> Option<String> {
        let mut all_records: HashMap<String, String> = HashMap::new();
        for record in self.get_reader().records() {
            let r = record.unwrap();
            all_records.insert(String::from(&r[0]), String::from(&r[1]));
        }

        let ret_val = all_records.remove(key);
        tracing::debug!(ret_val);

        tracing::debug!("{:?}", all_records);

        let mut writer = self.get_writer(false);
        for (key, value) in all_records {
            writer.write_record(&[key, value]).ok();
        }
        writer.flush().ok();

        ret_val
    }

    fn get_reader(&self) -> csv::Reader<std::fs::File> {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(self.filepath.clone())
            .unwrap()
    }

    fn get_writer(&self, append: bool) -> csv::Writer<std::fs::File> {
        let open_opts: &std::fs::OpenOptions;
        let mut blank_open_opts = std::fs::OpenOptions::new();
        if append {
            open_opts = blank_open_opts.create(true).append(true);
        } else {
            open_opts = blank_open_opts.create(true).write(true).truncate(true);
        }
        let file = open_opts.open(self.filepath.clone()).unwrap();
        csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file)
    }
}
