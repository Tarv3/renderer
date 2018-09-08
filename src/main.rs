extern crate renderer;

use renderer::test;

fn main() {
    let vertices = test::gen_plane();
    for vert in vertices {
        println!("{:?}", vert);
    }

}