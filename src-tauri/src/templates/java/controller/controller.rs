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

import {{package_name}}.util.Result;
import {{package_name}}.util.ResultPage;
import {{package_name}}.vo.req.{{class_name}}Req;
import {{package_name}}.vo.req.{{class_name}}ListReq;
import {{package_name}}.vo.req.{{class_name}}AddReq;
import {{package_name}}.vo.req.{{class_name}}UpdateReq;
import {{package_name}}.vo.resp.{{class_name}}Resp;
import {{package_name}}.service.{{class_name}}Service;

@Api(tags = \"{{table_comment}}\")
@RestController
@RequestMapping(\"/{{class_name_var}}\")
public class {{class_name}}Controller {

   @Autowired
   private {{class_name}}Service {{class_name_var}}Service;

   @ApiOperation(\"查询{{table_comment}}\")
   @PostMapping(\"/query\")
   public {{class_name}}Resp query(@RequestBody @Valid {{class_name}}Req record){
       return {{class_name_var}}Service.query(record);
   }

   @ApiOperation(\"查询{{table_comment}}列表\")
   @PostMapping(\"/list\")
   public Result<ResultPage<{{class_name}}Resp>> query{{class_name}}List(@RequestBody @Valid {{class_name}}ListReq record){
        return Result.success({{class_name_var}}Service.query{{class_name}}List(record));
   }

   @ApiOperation(\"添加{{table_comment}}\")
   @PostMapping(\"/add\")
   public Result<Integer> insert(@RequestBody @Valid {{class_name}}AddReq record){
        return Result.success({{class_name_var}}Service.insert(record));
   }

   @ApiOperation(\"删除{{table_comment}}\")
   @PostMapping(\"/delete\")
   public Result<Integer> delete(String ids){
        return Result.success({{class_name_var}}Service.delete(ids));
   }

   @ApiOperation(\"更新{{table_comment}}\")
   @PostMapping(\"/update\")
   public Result<Integer> update(@RequestBody @Valid {{class_name}}UpdateReq record){
        return Result.success({{class_name_var}}Service.update(record));
   }

}"
}
