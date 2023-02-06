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
public class SysMenu implements Serializable {

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

    //父ID
    private long parentId;

    //菜单名称
    private String menuName;

    //路由路径
    private String menuUrl;

    //接口URL
    private String apiUrl;

    //菜单图标
    private String menuIcon;

    //备注
    private String remark;

    //菜单类型(1：目录   2：菜单   3：按钮)
    private short menuType;

}
