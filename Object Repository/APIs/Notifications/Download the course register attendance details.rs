<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Download the course register attendance details</description>
   <name>Download the course register attendance details</name>
   <tag></tag>
   <elementGuidId>3a8fb720-93fd-4317-af55-a586f8706ac2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/vnd.ms-excel</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/faculty/DownloadAttendenceServlet?name=AttendenceStudents.xlsx&amp;operation=Attendencedata&amp;empName=C.%20Suguna%20Devi&amp;courseparam=201961010181722142801645&amp;param2=20-07-2020&amp;param3=03-08-2020&amp;empID=64DF070D8B6B11E98B0925647F276BC6&amp;coursecode=R17-7G134-A&amp;coursename=Discrete%20Mathematics&amp;deptname=05</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.URL</defaultValue>
      <description>url</description>
      <id>6879d222-7b9c-4bc9-b3b3-d27bab47a78e</id>
      <masked>false</masked>
      <name>URL</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
