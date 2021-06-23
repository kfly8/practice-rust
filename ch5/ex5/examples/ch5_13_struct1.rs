struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

fn main() {
    let trianble = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 1,
    };

    let vertexes = vec![(0, 0), (3, 0), (2, 2), (1, 2)];
    let fill = (0, 0, 0);
    let stroke_width = 1;
    let squrare = Polygon {
        vertexes,
        fill,
        stroke_width,
    };
}
