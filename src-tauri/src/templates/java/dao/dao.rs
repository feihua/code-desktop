pub fn get_dao() -> &'static str {
    "package {{package_name}}.dao;
import org.apache.ibatis.annotations.Mapper;

import java.util.List;

import {{package_name}}.entity.{{class_name}};

@Mapper
public interface {{class_name}}Dao {

   {{class_name}} query({{class_name}} {{class_name_var}});

   List<{{class_name}}> query{{class_name}}List({{class_name}} {{class_name_var}});

   int insert({{class_name}} {{class_name_var}});

   int delete(List<Integer> id);

   int update({{class_name}} {{class_name_var}});

}"
}
