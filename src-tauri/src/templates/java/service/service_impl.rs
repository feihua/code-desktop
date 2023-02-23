pub fn get_impl() -> &'static str {
    "package {{package_name}}.service.impl;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import org.springframework.beans.BeanUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import {{package_name}}.entity.{{class_name}};
import {{package_name}}.util.ResultPage;
import {{package_name}}.vo.req.{{class_name}}Req;
import {{package_name}}.vo.req.{{class_name}}ListReq;
import {{package_name}}.vo.req.{{class_name}}AddReq;
import {{package_name}}.vo.req.{{class_name}}UpdateReq;
import {{package_name}}.vo.resp.{{class_name}}Resp;
import {{package_name}}.dao.{{class_name}}Dao;
import {{package_name}}.service.{{class_name}}Service;
import com.github.pagehelper.PageHelper;
import com.github.pagehelper.PageInfo;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
@Service
public class {{class_name}}ServiceImpl implements {{class_name}}Service {

   @Autowired
   private {{class_name}}Dao {{class_name_var}}Dao;
   /**
    * 查询{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return {{class_name}}Resp
    * @author {{author}}
    * @date: {{current_time}}
    */
   @Override
   public {{class_name}}Resp query({{class_name}}Req {{class_name_var}}){

       {{class_name}} query = {{class_name_var}}Dao.query({{class_name}}.builder().build());

       return {{class_name}}Resp.builder().build();
   }
   /**
    * 查询{{table_comment}}列表
    *
    * @param {{class_name_var}} 请求参数
    * @return {{class_name}}Resp
    * @author {{author}}
    * @date: {{current_time}}
    */
   @Override
   public ResultPage<{{class_name}}Resp> query{{class_name}}List({{class_name}}ListReq {{class_name_var}}){

       PageHelper.startPage({{class_name_var}}.getCurrent(), {{class_name_var}}.getPageSize());
	   List<{{class_name}}> query = {{class_name_var}}Dao.query{{class_name}}List({{class_name}}.builder().build());
       PageInfo<{{class_name}}> pageInfo = new PageInfo<>(query);

	   List<{{class_name}}Resp> list = pageInfo.getList().stream().map(x -> {
		   {{class_name}}Resp resp = new {{class_name}}Resp();
		   BeanUtils.copyProperties(x, resp);
		   return resp;
	   }).collect(Collectors.toList());

        return new ResultPage<>(list,pageInfo.getPageNum(),pageInfo.getPageSize(),pageInfo.getTotal());

   }
   /**
    * 添加{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   @Override
   public int insert({{class_name}}AddReq {{class_name_var}}){

        return {{class_name_var}}Dao.insert({{class_name}}.builder().build());
   }
   /**
    * 删除{{table_comment}}
    *
    * @param ids 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   @Override
   public int delete(String ids){
		return {{class_name_var}}Dao.delete(Arrays.stream(ids.split(\",\")).map(Integer::parseInt).collect(Collectors.toList()));
   }
   /**
    * 更新{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   @Override
   public int update({{class_name}}UpdateReq {{class_name_var}}){

        return {{class_name_var}}Dao.update({{class_name}}.builder().build());
   }

}"
}