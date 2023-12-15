pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut plantable = 0;
    let mut just_planted = false;
    for (i, v) in flowerbed.iter().enumerate() {
        if *v == 0 {
            if just_planted {
                just_planted = false;
            }
            else {
                plantable += 1;
                just_planted = true;
            }
        }
        else {
            if just_planted {
                plantable -= 1;
            }
            just_planted = true;
        }
    }
    plantable >= n
}