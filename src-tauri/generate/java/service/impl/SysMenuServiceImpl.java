package com.example.springboottpl.service.impl;

import java.util.List;
import java.util.stream.Collectors;

import org.springframework.beans.BeanUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import com.example.springboottpl.entity.SysMenu;
import com.example.springboottpl.vo.req.SysMenuReq;
import com.example.springboottpl.vo.resp.SysMenuResp;
import com.example.springboottpl.dao.SysMenuDao;
import com.example.springboottpl.service.SysMenuService;

@Service
public class SysMenuServiceImpl implements SysMenuService {

   @Autowired
   private SysMenuDao recordDao;

   @Override
   public SysMenuResp query(SysMenuReq record){

       return null;
   }

   @Override
   public List<SysMenuResp> querySysMenuList(SysMenuReq record){
        SysMenu target = new SysMenu();
	   //BeanUtils.copyProperties(record,target);

	   List<SysMenu> query = recordDao.querySysMenuList(target);
	   return query.stream().map(x -> {
		   SysMenuResp resp = new SysMenuResp();
		   BeanUtils.copyProperties(x, resp);
		   return resp;
	   }).collect(Collectors.toList());
   }

   @Override
   public int insert(SysMenuReq record){
        SysMenu target = new SysMenu();

        return recordDao.insert(target);
   }

   @Override
   public int delete(int id){
        return recordDao.delete(id);
   }

   @Override
   public int update(SysMenuReq record){
        SysMenu target = new SysMenu();

        return recordDao.update(target);
   }

}