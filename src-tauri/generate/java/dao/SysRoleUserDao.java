package com.example.springboottpl.dao;
import org.apache.ibatis.annotations.Mapper;

import java.util.List;

import com.example.springboottpl.entity.SysRoleUser;

@Mapper
public interface SysRoleUserDao {

   SysRoleUser query(SysRoleUser record);

   List<SysRoleUser> querySysRoleUserList(SysRoleUser record);

   int insert(SysRoleUser record);

   int delete(int id);

   int update(SysRoleUser record);

}