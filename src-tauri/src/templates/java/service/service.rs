pub fn get_service() -> &'static str {
    "package {{package_name}}.service;

import java.util.Map;

import {{package_name}}.util.ResultPage;
import {{package_name}}.vo.req.{{class_name}}Req;
import {{package_name}}.vo.req.{{class_name}}ListReq;
import {{package_name}}.vo.req.{{class_name}}AddReq;
import {{package_name}}.vo.req.{{class_name}}UpdateReq;
import {{package_name}}.vo.resp.{{class_name}}Resp;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
public interface {{class_name}}Service {

   /**
    * 查询{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return {{class_name}}Resp
    * @author {{author}}
    * @date: {{current_time}}
    */
   {{class_name}}Resp query({{class_name}}Req {{class_name_var}});

   /**
    * 查询{{table_comment}}列表
    *
    * @param {{class_name_var}} 请求参数
    * @return ResultPage<{{class_name}}Resp>
    * @author {{author}}
    * @date: {{current_time}}
    */
   ResultPage<{{class_name}}Resp> query{{class_name}}List({{class_name}}ListReq {{class_name_var}});

   /**
    * 添加{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   int insert({{class_name}}AddReq {{class_name_var}});

   /**
    * 删除{{table_comment}}
    *
    * @param ids 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   int delete(String ids);

   /**
    * 更新{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   int update({{class_name}}UpdateReq {{class_name_var}});

}"
}