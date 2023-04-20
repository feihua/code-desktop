pub fn get_controller() -> &'static str {
    "package {{package_name}}.controller;

import io.swagger.annotations.Api;
import io.swagger.annotations.ApiOperation;

import java.util.Map;

import javax.validation.Valid;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.uaf.devops.common.util.Result;
import com.uaf.devops.common.util.ResultPage;
import {{package_name}}.vo.req.{{class_name}}ReqVo;
import {{package_name}}.vo.req.{{class_name}}ListReqVo;
import {{package_name}}.vo.req.{{class_name}}AddReqVo;
import {{package_name}}.vo.req.{{class_name}}UpdateReqVo;
import {{package_name}}.vo.resp.{{class_name}}RespVo;
import {{package_name}}.service.{{class_name}}Service;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
@Api(tags = \"{{table_comment}}\")
@RestController
@RequestMapping(\"/{{class_name_var}}\")
public class {{class_name}}Controller {

   @Autowired
   private {{class_name}}Service {{class_name_var}}Service;

   /**
    * 查询{{table_comment}}
    *
    * @param record 请求参数
    * @return {{class_name}}Resp
    * @author {{author}}
    * @date: {{current_time}}
    */
   @ApiOperation(\"查询{{table_comment}}\")
   @PostMapping(\"/query{{class_name}}\")
   public {{class_name}}RespVo query(@RequestBody @Valid {{class_name}}ReqVo record){
       return {{class_name_var}}Service.query{{class_name}}(record);
   }

   /**
    * 查询{{table_comment}}列表
    *
    * @param record 请求参数
    * @return {{class_name}}Resp
    * @author {{author}}
    * @date: {{current_time}}
    */
   @ApiOperation(\"查询{{table_comment}}列表\")
   @PostMapping(\"/query{{class_name}}List\")
   public Result<ResultPage<{{class_name}}RespVo>> query{{class_name}}List(@RequestBody @Valid {{class_name}}ListReqVo record){
        return Result.success({{class_name_var}}Service.query{{class_name}}List(record));
   }

   /**
    * 添加{{table_comment}}
    *
    * @param record 请求参数
    * @return Result<Integer>
    * @author {{author}}
    * @date: {{current_time}}
    */
   @ApiOperation(\"添加{{table_comment}}\")
   @PostMapping(\"/save{{class_name}}\")
   public Result<Integer> save{{class_name}}(@RequestBody @Valid {{class_name}}AddReqVo record){
        return Result.success({{class_name_var}}Service.save{{class_name}}(record));
   }

   /**
    * 删除{{table_comment}}
    *
    * @param ids 请求参数
    * @return Result<Integer>
    * @author {{author}}
    * @date: {{current_time}}
    */
   @ApiOperation(\"删除{{table_comment}}\")
   @PostMapping(\"/delete{{class_name}}\")
   public Result<Integer> delete{{class_name}}(String ids){
        return Result.success({{class_name_var}}Service.delete{{class_name}}(ids));
   }

   /**
    * 更新{{table_comment}}
    *
    * @param record 请求参数
    * @return Result<Integer>
    * @author {{author}}
    * @date: {{current_time}}
    */
   @ApiOperation(\"更新{{table_comment}}\")
   @PostMapping(\"/update{{class_name}}\")
   public Result<Integer> update{{class_name}}(@RequestBody @Valid {{class_name}}UpdateReqVo record){
        return Result.success({{class_name_var}}Service.update{{class_name}}(record));
   }

}"
}
