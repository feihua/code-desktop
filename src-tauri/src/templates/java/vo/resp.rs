pub fn get_resp() -> &'static str {
    "package {{package_name}}.vo.resp;

import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

import java.io.Serializable;
import java.util.Date;

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
@ApiModel(\"{{table_comment}}响应vo\")
public class {{class_name}}RespVo implements Serializable {
{% for column in java_columns %}
    @ApiModelProperty(\"{{column.column_comment}}\")
    private {{column.java_type}} {{column.java_name}};
{% endfor %}
}"
}
