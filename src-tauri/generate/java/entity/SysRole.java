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
public class SysRole implements Serializable {

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

    //名称
    private String roleName;

    //备注
    private String remark;

}
