use std::string::String;

use rsfml::graphics::{Color, RenderStates, RenderTarget, RenderWindow, Text, Texture, Vertex, Quads};
use rsfml::system::{Vector2f};

use assets::TextureId;
use asset_store::AssetStore;
use vec::Vec2f;

pub struct SfmlRenderer<'a> {
    target: &'a RenderTarget+'a,
    asset_store: &'a AssetStore,
}

impl<'a> SfmlRenderer<'a> {
    pub fn new(target: &'a RenderTarget, asset_store: &'a AssetStore) -> SfmlRenderer<'a> {
        SfmlRenderer {
            target: target,
            asset_store: asset_store,
        }
    }
    
    pub fn draw_text(&self, text: &Text) {
        self.target.draw_text(text);
    }
    
    pub fn draw_sf_texture(&self, texture: &Texture, x: f32, y: f32, rotation: f32) {        
        let size = texture.get_size();
        let (width, height) = (size.x as f32, size.y as f32);

        let vertices = [
            Vertex::new(&Vector2f{x: x, y: y}, &Color::white(), &Vector2f{x: 0f32, y: 0f32}),
            Vertex::new(&Vector2f{x: x, y: y + height}, &Color::white(), &Vector2f{x: 0f32, y: height}),
            Vertex::new(&Vector2f{x: x + width, y: y + height}, &Color::white(), &Vector2f{x: width, y: height}),
            Vertex::new(&Vector2f{x: x + width, y: y}, &Color::white(), &Vector2f{x: width, y: 0f32})
        ];
        
        let mut rs = RenderStates::default();
        rs.texture = Some(texture);
        rs.transform.rotate(rotation);
        
        self.target.draw_primitives_rs(&vertices, Quads, &mut rs);
    }
    
    pub fn draw_sf_texture_source(&self, texture: &Texture, x: f32, y: f32, rotation: f32, source_x: f32, source_y: f32, source_w: f32, source_h: f32) {        
        let size = texture.get_size();
        let (width, height) = (size.x as f32, size.y as f32);

        let vertices = [
            Vertex::new(&Vector2f{x: x, y: y}, &Color::white(), &Vector2f{x: source_x, y: source_y}),
            Vertex::new(&Vector2f{x: x, y: y + source_h}, &Color::white(), &Vector2f{x: source_x, y: source_y+source_h}),
            Vertex::new(&Vector2f{x: x + source_w, y: y + source_h}, &Color::white(), &Vector2f{x: source_x+source_w, y: source_y+source_h}),
            Vertex::new(&Vector2f{x: x + source_w, y: y}, &Color::white(), &Vector2f{x: source_x+source_w, y: source_y})
        ];
        
        let mut rs = RenderStates::default();
        rs.texture = Some(texture);
        rs.transform.rotate(rotation);
        
        self.target.draw_primitives_rs(&vertices, Quads, &mut rs);
    }
    
    pub fn draw_sf_texture_vec(&self, texture: &Texture, pos: &Vec2f, rotation: f32) {
        self.draw_sf_texture(texture, pos.x, pos.y, rotation);
    }
    
    pub fn draw_texture(&self, texture_id: TextureId, x: f32, y: f32, rotation: f32) {
        self.draw_sf_texture(self.asset_store.get_texture(texture_id), x, y, rotation);
    }
    
    pub fn draw_texture_vec(&self, texture_id: TextureId, pos: &Vec2f, rotation: f32) {
        self.draw_texture(texture_id, pos.x, pos.y, rotation);
    }
    
    pub fn draw_texture_source(&self, texture_id: TextureId, x: f32, y: f32, rotation: f32, source_x: f32, source_y: f32, source_w: f32, source_h: f32) {
        self.draw_sf_texture_source(self.asset_store.get_texture(texture_id), x, y, rotation, source_x, source_y, source_w, source_h);
    }
}