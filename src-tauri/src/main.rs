#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

pub mod model;
pub mod templates;

use std::path::PathBuf;
use chrono::Local;
use crate::model::db::{get_all_columns, get_columns, get_java_columns, get_table_comment};
use tera::{Tera, Context};
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use crate::templates::java::controller::controller::get_controller;
use crate::templates::java::dao::dao::get_dao;
use crate::templates::java::entity::entity::get_entity;
use crate::templates::java::mapper::mapper::get_mapper;
use crate::templates::java::react::components::add_form::get_react_add;
use crate::templates::java::react::components::search_form::get_react_search;
use crate::templates::java::react::components::update_form::get_react_update;
use crate::templates::java::react::data::get_react_data;
use crate::templates::java::react::index::get_react_index;
use crate::templates::java::react::r_service::get_react_service;
use crate::templates::java::service::service::get_service;
use crate::templates::java::service::service_impl::get_service_impl;
use crate::templates::java::biz::biz::get_biz;
use crate::templates::java::biz::biz_impl::get_biz_impl;
use crate::templates::java::vo::req::{get_add_req, get_list_req, get_req, get_update_req};
use crate::templates::java::vo::resp::get_resp;
use crate::templates::java::vue::components::add_form::get_vue_add;
use crate::templates::java::vue::components::list_table::get_vue_list;
use crate::templates::java::vue::components::update_form::get_vue_update;
use crate::templates::java::vue::data::get_vue_data;
use crate::templates::java::vue::index::get_vue_index;
use crate::templates::java::vue::v_service::get_vue_service;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_code(db_url: &str, db_name: &str, table_name: &str, package_name: &str, save_path: &str,
                 t_prefix: &str, font_path_name: &str, author: &str) -> String {

    // 待生成代码的表名
    // let table_name = "sys_user,sys_role_user,sys_role,sys_menu_role,sys_menu";

    let table_names: Vec<&str> = table_name.split(",").collect();

    for original_table_name in table_names {
        generate(db_url, db_name, original_table_name, package_name, save_path, t_prefix, font_path_name, author);
    };

    format!("Hello, {}! You've been greeted from Rust!", "table_name")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,generate_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate(db_url: &str, db_name: &str, original_table_name: &str, package_name: &str, save_path: &str,
            t_prefix: &str, font_path_name: &str, author: &str) {
    // 模板引擎
    let mut tera = match Tera::new("templates/**/*.*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    // 数据库地址
    // let url = "mysql://root:r-wz9wop62956dh5k9ed@rm-wz9a2yv489d123yqkdo.mysql.rds.aliyuncs.com:3306/information_schema";
    let mut url = String::from(db_url);
    url.push_str("/information_schema");

    // 待生成代码的数据库
    // let db_name = "rustdb";

    // 获取表字段
    let db_columns = get_columns(url.as_str(), db_name, original_table_name);
    if db_columns.clone().len() == 0 {
        println!("没有数据了");
        return;
    }
    // 把表字段转换成java字段
    let java_columns = get_java_columns(db_columns.clone());

    let all_columns = get_all_columns(db_columns);
    // 获取表注释
    let table_comment = get_table_comment(url.as_str(), db_name, original_table_name);

    let table_name_string = original_table_name.replace(t_prefix, "");
    let table_name = table_name_string.as_str();

    let binding = ToUpperCamelCase::to_upper_camel_case(table_name);
    // 类名
    let class_name = binding.as_str();
    let class_name_var = ToLowerCamelCase::to_lower_camel_case(class_name.clone());
    // 包名
    // let package_name = "com.example.springboottpl";

    let fmt = "%Y/%m/%d %H:%M:%S";
    let current_time = Local::now().format(fmt).to_string();

    let mut context = Context::new();
    context.insert("table_name", table_name);
    context.insert("original_table_name", original_table_name);
    context.insert("table_comment", table_comment.as_str());
    context.insert("package_name", package_name);
    context.insert("class_name", class_name);
    context.insert("class_name_var", class_name_var.as_str());
    context.insert("java_columns", &java_columns);
    context.insert("all_columns", all_columns.as_str());
    context.insert("current_time", current_time.as_str());
    context.insert("author", author);

    create_from_str(tera.clone(), class_name, &mut context, save_path);
    create_vue_from_str(tera.clone(), table_name, &mut context, font_path_name);
    create_react_from_str(tera.clone(), table_name, &mut context, font_path_name);
}

fn create_from_str(tera: Tera, class_name: &str, mut context: &mut Context, save_path: &str) {
    write_str_file(tera.clone(), &mut context, get_entity(), format!("entity/{}Bean.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_controller(), format!("controller/{}Controller.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_dao(), format!("dao/{}Dao.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_mapper(), format!("mapper/{}Mapper.xml", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_service(), format!("service/{}Service.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_service_impl(), format!("service/impl/{}ServiceImpl.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_biz(), format!("biz/{}Biz.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_biz_impl(), format!("biz/impl/{}BizImpl.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_req(), format!("vo/req/{}ReqVo.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_list_req(), format!("vo/req/{}ListReqVo.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_add_req(), format!("vo/req/{}AddReqVo.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_update_req(), format!("vo/req/{}UpdateReqVo.java", class_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_resp(), format!("vo/resp/{}RespVo.java", class_name).as_str(), save_path);
}

fn create_vue_from_str(tera: Tera, table_name: &str, mut context: &mut Context, save_path: &str) {
    write_str_file(tera.clone(), &mut context, get_vue_index(), format!("vue/{}/index.vue", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_vue_data(), format!("vue/{}/data.d.ts", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_vue_service(), format!("vue/{}/service.ts", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_vue_add(), format!("vue/{}/components/AddForm.vue", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_vue_list(), format!("vue/{}/components/ListTable.vue", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_vue_update(), format!("vue/{}/components/UpdateForm.vue", table_name).as_str(), save_path);
}

fn create_react_from_str(tera: Tera, table_name: &str, mut context: &mut Context, save_path: &str) {
    write_str_file(tera.clone(), &mut context, get_react_index(), format!("react/{}/index.tsx", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_react_data(), format!("react/{}/data.d.ts", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_react_service(), format!("react/{}/service.ts", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_react_add(), format!("react/{}/components/AddForm.tsx", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_react_search(), format!("react/{}/components/SearchForm.tsx", table_name).as_str(), save_path);
    write_str_file(tera.clone(), &mut context, get_react_update(), format!("react/{}/components/UpdateForm.tsx", table_name).as_str(), save_path);
}

fn write_str_file(mut tera: Tera, context: &mut Context, template_file_str: &str, target_file_name: &str, save_path: &str) {
    let result = tera.render_str(template_file_str, &context);

    // let cwd = std::env::current_dir().unwrap();
    // let folder = cwd.as_path().join("generate").join(target_file_name);
    let folder;
    if save_path == "" {
        folder = PathBuf::from(format!("E:\\generate\\{}", target_file_name));
    } else {
        let mut path = String::from(save_path);
        path.push_str(target_file_name);
        folder = PathBuf::from(path);
    }

    let r = result.unwrap();
    println!("create -------------->{:?}", folder);
    let path = folder.parent().unwrap();
    std::fs::create_dir_all(path).unwrap();
    let mut file = std::fs::File::create(folder).unwrap();
    std::io::Write::write_all(&mut file, r.as_ref()).unwrap();
}