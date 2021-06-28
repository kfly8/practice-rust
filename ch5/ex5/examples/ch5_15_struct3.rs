struct Triangle(Vertex, Vertex, Vertex);
struct Vertex(i32, i32);

fn main() {
    let p0 = Vertex(0, 0);
    let p1 = Vertex(3, 0);
    let triangle = Triangle(p0, p1, Vertex(2, 2));

    assert_eq!((triangle.1).0, 3);
    assert_eq!((triangle.2).1, 2);
}
