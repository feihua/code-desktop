pub fn get_biz_impl() -> &'static str {
    "package {{package_name}}.biz.impl;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import {{package_name}}.entity.{{class_name}}Bean;
import com.uaf.devops.common.util.ResultPage;
import {{package_name}}.vo.req.{{class_name}}ReqVo;
import {{package_name}}.vo.req.{{class_name}}ListReqVo;
import {{package_name}}.vo.req.{{class_name}}AddReqVo;
import {{package_name}}.vo.req.{{class_name}}UpdateReqVo;
import {{package_name}}.vo.resp.{{class_name}}RespVo;
import {{package_name}}.dao.{{class_name}}Dao;
import {{package_name}}.biz.{{class_name}}Biz;
import com.github.pagehelper.PageHelper;
import com.github.pagehelper.PageInfo;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
@Service
public class {{class_name}}BizImpl implements {{class_name}}Biz {

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
   public {{class_name}}RespVo query{{class_name}}({{class_name}}ReqVo {{class_name_var}}){
        {{class_name}}Bean bean = new {{class_name}}Bean();{% for column in java_columns %}
        //bean.set{{column.java_name_letter}}({{class_name_var}}.get{{column.java_name_letter}}());{% endfor %}

        {{class_name}}Bean query = {{class_name_var}}Dao.query{{class_name}}(bean);

        return {{class_name}}RespVo.builder().build();
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
   public ResultPage<{{class_name}}RespVo> query{{class_name}}List({{class_name}}ListReqVo {{class_name_var}}){
        {{class_name}}Bean bean = new {{class_name}}Bean();{% for column in java_columns %}
        //bean.set{{column.java_name_letter}}({{class_name_var}}.get{{column.java_name_letter}}());{% endfor %}

        PageHelper.startPage({{class_name_var}}.getPageNum(), {{class_name_var}}.getPageSize());
	    List<{{class_name}}Bean> query = {{class_name_var}}Dao.query{{class_name}}List(bean);
        PageInfo<{{class_name}}Bean> pageInfo = new PageInfo<>(query);

	    List<{{class_name}}RespVo> list = pageInfo.getList().stream().map(x -> {
            {{class_name}}RespVo resp = new {{class_name}}RespVo();{% for column in java_columns %}
            resp.set{{column.java_name_letter}}(x.get{{column.java_name_letter}}());{% endfor %}
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
   public int save{{class_name}}({{class_name}}AddReqVo {{class_name_var}}){
        {{class_name}}Bean bean = new {{class_name}}Bean();{% for column in java_columns %}
        bean.set{{column.java_name_letter}}({{class_name_var}}.get{{column.java_name_letter}}());{% endfor %}

        return {{class_name_var}}Dao.save{{class_name}}(bean);
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
   public int delete{{class_name}}(String ids){
		return {{class_name_var}}Dao.delete{{class_name}}(Arrays.stream(ids.split(\",\")).map(Integer::parseInt).collect(Collectors.toList()));
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
   public int update{{class_name}}({{class_name}}UpdateReqVo {{class_name_var}}){
        {{class_name}}Bean bean = new {{class_name}}Bean();{% for column in java_columns %}
        bean.set{{column.java_name_letter}}({{class_name_var}}.get{{column.java_name_letter}}());{% endfor %}

        return {{class_name_var}}Dao.update{{class_name}}(bean);
   }

}"
}