pub struct Race {
    _time: i64,
    _distance: i64
}
impl Default for Race {
    fn default() -> Self {
        Self { _time: 0, _distance: 0 }
    }
}
impl Race {
    pub fn get_time(self: &Self) -> i64 {self._time}
    pub fn get_distance(self: &Self) -> i64 {self._distance}

    pub fn create (time: i64, distance: i64) -> Self {
        Self {_time: time, _distance: distance}
    }
}

pub fn create_races(file_contents: &String) -> Vec<Race> {

    let result1: Vec<_> = file_contents.split('\n').nth(0).unwrap().split(':').nth(1).unwrap().split(' ').filter(|p|!p.is_empty()).collect();
    let result2: Vec<_> = file_contents.split('\n').nth(1).unwrap().split(':').nth(1).unwrap().split(' ').filter(|p|!p.is_empty()).collect();


    let mut races:Vec<Race> = vec![];
    for index in 0..result1.len() {
        races.push(
            Race
            {
                _time: (*result1.get(index).unwrap()).trim().parse::<i64>().unwrap(),
                _distance: (*result2.get(index).unwrap()).trim().parse::<i64>().unwrap()
            }
        );
    }


    return races;
}