package com.example.springboottpl.dao;
import org.apache.ibatis.annotations.Mapper;

import java.util.List;

import com.example.springboottpl.entity.SysMenu;

@Mapper
public interface SysMenuDao {

   SysMenu query(SysMenu record);

   List<SysMenu> querySysMenuList(SysMenu record);

   int insert(SysMenu record);

   int delete(int id);

   int update(SysMenu record);

}