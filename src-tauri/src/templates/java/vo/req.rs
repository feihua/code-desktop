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

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@ApiModel(\"{{table_comment}}请求vo\")
public class {{class_name}}Req implements Serializable {
{% for column in java_columns %}
    @ApiModelProperty(\"{{column.column_comment}}\")
    //@NotNull(message = \"{{column.java_name}}{{column.column_comment}}不能为空\")
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

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@ApiModel(\"{{table_comment}}请求listVVo\")
public class {{class_name}}ListReq implements Serializable {

    @ApiModelProperty(\"当前页\")
    @NotNull(message = \"current当前页不能为空\")
    @Min(value=1,message = \"current当前页不能小于1\")
    private int current;

    @ApiModelProperty(\"每页的数量\")
    @NotNull(message = \"pageSize每页的数量不能为空\")
    private int pageSize;
{% for column in java_columns %}
    @ApiModelProperty(\"{{column.column_comment}}\")
    //@NotNull(message = \"{{column.java_name}}{{column.column_comment}}不能为空\")
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

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@ApiModel(\"{{table_comment}}请求addVo\")
public class {{class_name}}AddReq implements Serializable {
{% for column in java_columns %}
    @ApiModelProperty(\"{{column.column_comment}}\")
    //@NotNull(message = \"{{column.java_name}}{{column.column_comment}}不能为空\")
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

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@ApiModel(\"{{table_comment}}请求updateVo\")
public class {{class_name}}UpdateReq implements Serializable {
{% for column in java_columns %}
    @ApiModelProperty(\"{{column.column_comment}}\")
    //@NotNull(message = \"{{column.java_name}}{{column.column_comment}}不能为空\")
    private {{column.java_type}} {{column.java_name}};
{% endfor %}
}"
}
