package com.example.springboottpl.dao;
import org.apache.ibatis.annotations.Mapper;

import java.util.List;

import com.example.springboottpl.entity.SysUser;

@Mapper
public interface SysUserDao {

   SysUser query(SysUser record);

   List<SysUser> querySysUserList(SysUser record);

   int insert(SysUser record);

   int delete(int id);

   int update(SysUser record);

}