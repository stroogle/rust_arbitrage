use crate::Path;

pub struct PathList {

}

impl PathList {
    pub fn get_path_vec(path: &str) -> Vec<Path> {
        // let mut map: HashMap<String, Ticker> = HashMap::new();
        let mut vec: Vec<Path> = Vec::new();

        let file_contents = PathList::read_file(&path);

        for line in file_contents.lines() {

            // let assets: Vec<&str> = line.split("|").collect();
            // let ticker = Ticker::new(assets[0], assets[1]);
            // map.insert(ticker.symbol.clone(), ticker);
        }

        return vec;

    }

    fn read_file(path: &str) -> String {
        fs::read_to_string(path).unwrap()
    }
}