package com.example.springboottpl.vo.req;

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
@ApiModel("菜单角色关联表请求vo")
public class SysMenuRoleReq implements Serializable {

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

    @ApiModelProperty("菜单ID")
    private long menuId;

    @ApiModelProperty("角色ID")
    private long roleId;

}