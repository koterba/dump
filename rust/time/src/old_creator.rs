pub fn segment_creator(
        top: bool, 
        top_left: bool,
        top_right: bool,
        middle: bool,
        bottom_left: bool,
        bottom_right: bool,
        bottom: bool ) -> Vec<String>
    {
    let top_line = match top {
        true  => "   _ _   ",
        false => "         "
    };
    let second_third_line = match (top_left, top_right) {
        (true, true)   => " *     * ",
        (true, false)  => " *       ",
        (false, true)  => "       * ",
        (false, false) => "         ",
    };
    let middle_line = match middle {
        true  => "   * *   ",
        false => "         "
    };
    let fifth_sixth_line = match (bottom_left, bottom_right) {
        (true, true)   => " *     * ",
        (true, false)  => " *       ",
        (false, true)  => "       * ",
        (false, false) => "         ",
    };
    let bottom_line = match bottom {
        true  => "   * *   ",
        false => "         "
    };

    let result: Vec<String> = vec![
    top_line.to_string(),
    second_third_line.to_string(),
    second_third_line.to_string(),
    middle_line.to_string(),
    fifth_sixth_line.to_string(),
    fifth_sixth_line.to_string(),
    bottom_line.to_string()];

    result
}
