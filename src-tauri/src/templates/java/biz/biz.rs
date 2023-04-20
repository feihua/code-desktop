pub fn get_biz() -> &'static str {
    "package {{package_name}}.biz;

import java.util.Map;

import com.uaf.devops.common.util.ResultPage;
import {{package_name}}.vo.req.{{class_name}}ReqVo;
import {{package_name}}.vo.req.{{class_name}}ListReqVo;
import {{package_name}}.vo.req.{{class_name}}AddReqVo;
import {{package_name}}.vo.req.{{class_name}}UpdateReqVo;
import {{package_name}}.vo.resp.{{class_name}}RespVo;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
public interface {{class_name}}Biz {

   /**
    * 查询{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return {{class_name}}Resp
    * @author {{author}}
    * @date: {{current_time}}
    */
   {{class_name}}RespVo query{{class_name}}({{class_name}}ReqVo {{class_name_var}});

   /**
    * 查询{{table_comment}}列表
    *
    * @param {{class_name_var}} 请求参数
    * @return ResultPage<{{class_name}}Resp>
    * @author {{author}}
    * @date: {{current_time}}
    */
   ResultPage<{{class_name}}RespVo> query{{class_name}}List({{class_name}}ListReqVo {{class_name_var}});

   /**
    * 添加{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   int save{{class_name}}({{class_name}}AddReqVo {{class_name_var}});

   /**
    * 删除{{table_comment}}
    *
    * @param ids 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   int delete{{class_name}}(String ids);

   /**
    * 更新{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   int update{{class_name}}({{class_name}}UpdateReqVo {{class_name_var}});

}"
}