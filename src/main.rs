mod line;
mod framebuffers;
mod flood_fill;

use raylib::prelude::*;
use framebuffers::Framebuffer;
use line::line;
use flood_fill::flood_fill_vector;


fn main() {
   
    let mut fb = Framebuffer::new(1000, 1000, Color::BLACK);

    fb.set_current_color(Color::RED);
    
    fb.set_current_color(Color::ORANGE);
    //uno

    line(&mut fb, Vector2::new(165.0, 380.0), Vector2::new(185.0, 360.0));
    line(&mut fb, Vector2::new(185.0, 360.0), Vector2::new(180.0, 330.0));
    line(&mut fb, Vector2::new(180.0, 330.0), Vector2::new(207.0, 345.0));
    line(&mut fb, Vector2::new(207.0, 345.0), Vector2::new(233.0, 330.0));
    line(&mut fb, Vector2::new(233.0, 330.0), Vector2::new(230.0, 360.0));
    line(&mut fb, Vector2::new(230.0, 360.0), Vector2::new(250.0, 380.0));
    line(&mut fb, Vector2::new(250.0, 380.0), Vector2::new(220.0, 385.0));
    line(&mut fb, Vector2::new(220.0, 385.0), Vector2::new(205.0, 410.0));
    line(&mut fb, Vector2::new(205.0, 410.0), Vector2::new(193.0, 383.0));
    line(&mut fb, Vector2::new(193.0, 383.0), Vector2::new(165.0, 380.0));

    //Dos

    fb.set_current_color(Color::BLUE);
    line(&mut fb, Vector2::new(321.0, 335.0), Vector2::new(288.0, 286.0));
    line(&mut fb, Vector2::new(288.0, 286.0), Vector2::new(339.0, 251.0));
    line(&mut fb, Vector2::new(339.0, 251.0), Vector2::new(374.0, 302.0));
    line(&mut fb, Vector2::new(374.0, 302.0), Vector2::new(321.0, 335.0));

    //Tres

     fb.set_current_color(Color::VIOLET);
    line(&mut fb, Vector2::new(377.0, 249.0), Vector2::new(411.0, 197.0));
    line(&mut fb, Vector2::new(411.0, 197.0), Vector2::new(436.0, 249.0));
    line(&mut fb, Vector2::new(436.0, 249.0), Vector2::new(377.0, 249.0));

    //Cuatro

    fb.set_current_color(Color::BLUEVIOLET);
    line(&mut fb, Vector2::new(413.0, 177.0), Vector2::new(448.0, 159.0));
    line(&mut fb, Vector2::new(448.0, 159.0), Vector2::new(502.0, 88.0));
    line(&mut fb, Vector2::new(502.0, 88.0), Vector2::new(553.0, 53.0));
    line(&mut fb, Vector2::new(553.0, 53.0), Vector2::new(535.0, 36.0));
    line(&mut fb, Vector2::new(535.0, 36.0), Vector2::new(676.0, 37.0));
    line(&mut fb, Vector2::new(676.0, 37.0), Vector2::new(660.0, 52.0));
    line(&mut fb, Vector2::new(660.0, 52.0), Vector2::new(750.0, 145.0));
    line(&mut fb, Vector2::new(750.0, 145.0), Vector2::new(761.0, 179.0));
    line(&mut fb, Vector2::new(761.0, 179.0), Vector2::new(672.0, 192.0));
    line(&mut fb, Vector2::new(672.0, 192.0), Vector2::new(659.0, 214.0));
    line(&mut fb, Vector2::new(659.0, 214.0), Vector2::new(615.0, 214.0));
    line(&mut fb, Vector2::new(615.0, 214.0), Vector2::new(632.0, 230.0));
    line(&mut fb, Vector2::new(632.0, 230.0), Vector2::new(580.0, 230.0));
    line(&mut fb, Vector2::new(580.0, 230.0), Vector2::new(597.0, 215.0));
    line(&mut fb, Vector2::new(597.0, 215.0), Vector2::new(552.0, 214.0));
    line(&mut fb, Vector2::new(552.0, 214.0), Vector2::new(517.0, 144.0));
    line(&mut fb, Vector2::new(517.0, 144.0), Vector2::new(466.0, 180.0));
    line(&mut fb, Vector2::new(466.0, 180.0), Vector2::new(413.0, 177.0));

    //Cinco

    fb.set_current_color(Color::BLUEVIOLET);
    line(&mut fb, Vector2::new(682.0, 175.0), Vector2::new(708.0, 120.0));
    line(&mut fb, Vector2::new(708.0, 120.0), Vector2::new(735.0, 148.0));
    line(&mut fb, Vector2::new(735.0, 148.0), Vector2::new(739.0, 170.0));
    line(&mut fb, Vector2::new(739.0, 170.0), Vector2::new(682.0, 175.0));

    

    flood_fill_vector(&mut fb, Vector2::new(200.0, 365.0), Color::new(255, 100, 100, 180));
    flood_fill_vector(&mut fb, Vector2::new(330.0, 300.0), Color::new(173, 216, 230, 200));
    flood_fill_vector(&mut fb, Vector2::new(408.0, 230.0), Color::new(221, 160, 221, 200));
    flood_fill_vector(&mut fb, Vector2::new(600.0, 120.0), Color::new(138, 43, 226, 150));

    
    fb.render_to_file("output.bmp");
}