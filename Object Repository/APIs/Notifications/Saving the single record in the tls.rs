<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For Saving the single record in the tls</description>
   <name>Saving the single record in the tls</name>
   <tag></tag>
   <elementGuidId>4965bcd3-1532-4413-9f29-c7c20242d8d9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/faculty/saveLectureSchedule?jsonData=%7B%22lecturescheduleid%22%3A%22%22%2C%22employee%22%3A%7B%22employeePrimaryId%22%3A%2264DF070D8B6B11E98B0925647F276BC6%22%7D%2C%22acadReg%22%3A%7B%22academicID%22%3A%22F64FCD564B0B4A6486EA41966CC4D9C1%22%7D%2C%22course%22%3A%7B%22coursePrimaryId%22%3A%22201961010181722142801645%22%7D%2C%22topic%22%3A%22Introduction%22%2C%22unit%22%3A%221%22%2C%22subjectProgramOutcome%22%3A%22%22%2C%22stratagy%22%3A%22ss%22%2C%22referencePages%22%3A%221-2%22%2C%22courseOutcome%22%3A%22CO1%22%2C%22bloomsLevel%22%3A%22L2%22%2C%22semid%22%3A%2220208374864487839328%22%2C%22coursecode%22%3A%22R17-7G134-A%22%2C%22praposed%22%3A%222020-07-23%2000%3A00%3A00.0%22%7D</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.URL</defaultValue>
      <description>url</description>
      <id>f6d315a8-6ced-4ee9-929c-de965e92c60f</id>
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
