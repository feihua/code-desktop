package com.example.springboottpl.dao;
import org.apache.ibatis.annotations.Mapper;

import java.util.List;

import com.example.springboottpl.entity.SysMenuRole;

@Mapper
public interface SysMenuRoleDao {

   SysMenuRole query(SysMenuRole record);

   List<SysMenuRole> querySysMenuRoleList(SysMenuRole record);

   int insert(SysMenuRole record);

   int delete(int id);

   int update(SysMenuRole record);

}