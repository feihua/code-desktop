pub fn get_req() -> &'static str {
    "package {{package_name}}.vo.req;

import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

import java.io.Serializable;
import java.util.Date;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@ApiModel(\"{{table_comment}}请求vo\")
public class {{class_name}}ReqVo implements Serializable {
{% for column in java_columns %}
    @ApiModelProperty(\"{{column.column_comment}}\")
    //@NotBlank(message = \"{{column.java_name}}{{column.column_comment}}不能为空\")
    private {{column.java_type}} {{column.java_name}};
{% endfor %}
}"
}

pub fn get_list_req() -> &'static str {
    "package {{package_name}}.vo.req;

import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

import java.io.Serializable;
import java.util.Date;

import javax.validation.constraints.Min;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@ApiModel(\"{{table_comment}}请求listVo\")
public class {{class_name}}ListReqVo implements Serializable {

    @ApiModelProperty(\"当前页\")
    @NotNull(message = \"pageNum当前页不能为空\")
    @Min(value=1,message = \"pageNum当前页不能小于1\")
    private Integer pageNum;

    @ApiModelProperty(\"每页的数量\")
    @NotNull(message = \"pageSize每页的数量不能为空\")
    private Integer pageSize;

{% for column in java_columns %}
    @ApiModelProperty(\"{{column.column_comment}}\")
    //@NotBlank(message = \"{{column.java_name}}{{column.column_comment}}不能为空\")
    private {{column.java_type}} {{column.java_name}};
{% endfor %}
}"
}

pub fn get_add_req() -> &'static str {
    "package {{package_name}}.vo.req;

import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

import java.io.Serializable;
import java.util.Date;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@ApiModel(\"{{table_comment}}请求addVo\")
public class {{class_name}}AddReqVo implements Serializable {
{% for column in java_columns %}
    @ApiModelProperty(\"{{column.column_comment}}\")
    //@NotBlank(message = \"{{column.java_name}}{{column.column_comment}}不能为空\")
    private {{column.java_type}} {{column.java_name}};
{% endfor %}
}"
}

pub fn get_update_req() -> &'static str {
    "package {{package_name}}.vo.req;

import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

import java.io.Serializable;
import java.util.Date;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * 描述：{{table_comment}}
 * 作者：{{author}}
 * 日期：{{current_time}}
 */
@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@ApiModel(\"{{table_comment}}请求updateVo\")
public class {{class_name}}UpdateReqVo implements Serializable {
{% for column in java_columns %}
    @ApiModelProperty(\"{{column.column_comment}}\")
    //@NotBlank(message = \"{{column.java_name}}{{column.column_comment}}不能为空\")
    private {{column.java_type}} {{column.java_name}};
{% endfor %}
}"
}
