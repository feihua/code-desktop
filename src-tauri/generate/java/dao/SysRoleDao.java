package com.example.springboottpl.dao;
import org.apache.ibatis.annotations.Mapper;

import java.util.List;

import com.example.springboottpl.entity.SysRole;

@Mapper
public interface SysRoleDao {

   SysRole query(SysRole record);

   List<SysRole> querySysRoleList(SysRole record);

   int insert(SysRole record);

   int delete(int id);

   int update(SysRole record);

}