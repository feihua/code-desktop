package com.example.springboottpl.entity;

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
public class SysMenuRole implements Serializable {

    //主键
    private long id;

    //创建时间
    private Date gmtCreate;

    //修改时间
    private Date gmtModified;

    //状态(1:正常，0:禁用)
    private short statusId;

    //排序
    private int sort;

    //菜单ID
    private long menuId;

    //角色ID
    private long roleId;

}
