package com.example.springboottpl.vo.resp;

import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;

import java.io.Serializable;
import java.util.Date;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@ApiModel("后台用户信息响应vo")
public class SysUserResp implements Serializable {

    @ApiModelProperty("主键")
    private long id;

    @ApiModelProperty("创建时间")
    private Date gmtCreate;

    @ApiModelProperty("修改时间")
    private Date gmtModified;

    @ApiModelProperty("状态(1:正常，0:禁用)")
    private short statusId;

    @ApiModelProperty("排序")
    private int sort;

    @ApiModelProperty("用户编号")
    private long userNo;

    @ApiModelProperty("手机")
    private String mobile;

    @ApiModelProperty("真实姓名")
    private String realName;

    @ApiModelProperty("备注")
    private String remark;

    @ApiModelProperty("密码")
    private String password;

}