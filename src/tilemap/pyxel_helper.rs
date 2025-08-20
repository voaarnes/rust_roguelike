// Helper to visualize Pyxel Edit tile indices
pub fn print_tile_grid() {
    println!("Pyxel Edit Tile Index Reference (16x16 grid):");
    println!("=" .repeat(80));
    
    for row in 0..16 {
        for col in 0..16 {
            let index = row * 16 + col;
            print!("{:3} ", index);
        }
        println!();
    }
    
    println!("\n" + "=" .repeat(80));
    println!("Formula: index = row * 16 + column");
    println!("Example: Tile at row 2, column 5 = 2*16 + 5 = 37");
}

// Call this in your startup system to see the grid
pub fn debug_tile_indices(
) {
    print_tile_grid();
}
