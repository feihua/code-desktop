pub fn get_service() -> &'static str {
    "package {{package_name}}.service;

import java.util.Map;

import {{package_name}}.vo.req.{{class_name}}Req;
import {{package_name}}.vo.req.{{class_name}}ListReq;
import {{package_name}}.vo.req.{{class_name}}AddReq;
import {{package_name}}.vo.req.{{class_name}}UpdateReq;
import {{package_name}}.vo.resp.{{class_name}}Resp;

public interface {{class_name}}Service {

   {{class_name}}Resp query({{class_name}}Req record);

   Map<String,Object> query{{class_name}}List({{class_name}}ListReq {{class_name_var}});

   int insert({{class_name}}AddReq {{class_name_var}});

   int delete(String ids);

   int update({{class_name}}UpdateReq {{class_name_var}});

}"
}