pub fn get_mapper() -> &'static str {
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE mapper PUBLIC \"-//mybatis.org//DTD Mapper 3.0//EN\" \"https://mybatis.org/dtd/mybatis-3-mapper.dtd\">
    <mapper namespace=\"{{package_name}}.dao.{{class_name}}Dao\">

    <resultMap id=\"BaseResultMap\" type=\"{{package_name}}.entity.{{class_name}}Bean\">{% for column in java_columns %}
        <result column=\"{{column.db_name}}\" property=\"{{column.java_name}}\" jdbcType=\"{{column.jdbc_type}}\"/>{% endfor %}
    </resultMap>

    <sql id=\"Base_Column_List\">
        {{all_columns}}
    </sql>

    <select id=\"query{{class_name}}\" parameterType=\"{{package_name}}.entity.{{class_name}}Bean\" resultMap=\"BaseResultMap\">
        select
        <include refid=\"Base_Column_List\"/>
        from {{original_table_name}}
        <where> {% for column in java_columns %}
            <!--<if test=\"{{column.java_name}} != null\">-->
            <!--    and {{column.db_name}} = #{ {{column.java_name}}}-->
            <!--</if>-->{% endfor %}
        </where>
    </select>

    <select id=\"query{{class_name}}List\" parameterType=\"{{package_name}}.entity.{{class_name}}Bean\" resultMap=\"BaseResultMap\">
        select
        <include refid=\"Base_Column_List\"/>
        from {{original_table_name}}
        <where> {% for column in java_columns %}
            <!--<if test=\"{{column.java_name}} != null\">-->
            <!--    and {{column.db_name}} = #{ {{column.java_name}}}-->
            <!--</if>-->{% endfor %}
        </where>
    </select>

    <insert id=\"save{{class_name}}\" parameterType=\"{{package_name}}.entity.{{class_name}}Bean\">
        insert into {{original_table_name}}
        <trim prefix=\"(\" suffix=\")\" suffixOverrides=\",\">{% for column in java_columns %}
            <if test=\"{{column.java_name}} != null\">
                {{column.db_name}},
            </if>{% endfor %}
        </trim>
        <trim prefix=\"values (\" suffix=\")\" suffixOverrides=\",\">{% for column in java_columns %}
            <if test=\"{{column.java_name}} != null\">
                #{ {{column.java_name}},jdbcType={{column.jdbc_type}} },
            </if>{% endfor %}
        </trim>
    </insert>

    <delete id=\"delete{{class_name}}\">
        delete from {{original_table_name}} where id in
        <foreach collection=\"list\" item=\"id\" index=\"index\"
            open=\"(\" close=\")\" separator=\",\">
            #{id}
        </foreach>
    </delete>

    <update id=\"update{{class_name}}\" parameterType=\"{{package_name}}.entity.{{class_name}}Bean\">
        update {{original_table_name}}
        <set>{% for column in java_columns %}
            <if test=\"{{column.java_name}} != null\">
                {{column.db_name}} = #{ {{column.java_name}},jdbcType={{column.jdbc_type}}},
            </if>{% endfor %}
        </set>
        <where> {% for column in java_columns %}
            <if test=\"{{column.java_name}} != null\">
                and {{column.db_name}} = #{ {{column.java_name}}}
            </if>{% endfor %}
        </where>
    </update>

    </mapper>"
}