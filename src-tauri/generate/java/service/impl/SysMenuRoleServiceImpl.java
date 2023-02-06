package com.example.springboottpl.service.impl;

import java.util.List;
import java.util.stream.Collectors;

import org.springframework.beans.BeanUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import com.example.springboottpl.entity.SysMenuRole;
import com.example.springboottpl.vo.req.SysMenuRoleReq;
import com.example.springboottpl.vo.resp.SysMenuRoleResp;
import com.example.springboottpl.dao.SysMenuRoleDao;
import com.example.springboottpl.service.SysMenuRoleService;

@Service
public class SysMenuRoleServiceImpl implements SysMenuRoleService {

   @Autowired
   private SysMenuRoleDao recordDao;

   @Override
   public SysMenuRoleResp query(SysMenuRoleReq record){

       return null;
   }

   @Override
   public List<SysMenuRoleResp> querySysMenuRoleList(SysMenuRoleReq record){
        SysMenuRole target = new SysMenuRole();
	   //BeanUtils.copyProperties(record,target);

	   List<SysMenuRole> query = recordDao.querySysMenuRoleList(target);
	   return query.stream().map(x -> {
		   SysMenuRoleResp resp = new SysMenuRoleResp();
		   BeanUtils.copyProperties(x, resp);
		   return resp;
	   }).collect(Collectors.toList());
   }

   @Override
   public int insert(SysMenuRoleReq record){
        SysMenuRole target = new SysMenuRole();

        return recordDao.insert(target);
   }

   @Override
   public int delete(int id){
        return recordDao.delete(id);
   }

   @Override
   public int update(SysMenuRoleReq record){
        SysMenuRole target = new SysMenuRole();

        return recordDao.update(target);
   }

}