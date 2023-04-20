pub fn get_dao() -> &'static str {
    "package {{package_name}}.dao;
import org.apache.ibatis.annotations.Mapper;

import java.util.List;

import {{package_name}}.entity.{{class_name}}Bean;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
@Mapper
public interface {{class_name}}Dao {

   /**
    * 查询{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return {{class_name}}
    * @author {{author}}
    * @date: {{current_time}}
    */
   {{class_name}}Bean query{{class_name}}({{class_name}}Bean {{class_name_var}});

   /**
    * 查询{{table_comment}}列表
    *
    * @param {{class_name_var}} 请求参数
    * @return List<{{class_name}}>
    * @author {{author}}
    * @date: {{current_time}}
    */
   List<{{class_name}}Bean> query{{class_name}}List({{class_name}}Bean {{class_name_var}});

   /**
    * 添加{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   int save{{class_name}}({{class_name}}Bean {{class_name_var}});

   /**
    * 删除{{table_comment}}
    *
    * @param ids 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   int delete{{class_name}}(List<Integer> ids);

   /**
    * 更新{{table_comment}}
    *
    * @param {{class_name_var}} 请求参数
    * @return int
    * @author {{author}}
    * @date: {{current_time}}
    */
   int update{{class_name}}({{class_name}}Bean {{class_name_var}});

}"
}
