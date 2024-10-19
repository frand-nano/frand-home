mod index;
mod login;
mod ws;
mod res;

pub use self::{
    index::{get_index, get_index_favicon, get_index_path},
    login::get_login,
    ws::get_ws,
    res::get_res_path,
};