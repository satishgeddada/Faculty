<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For add the single phd details</description>
   <name>Add the single phd details</name>
   <tag></tag>
   <elementGuidId>d0314614-9823-401d-a759-dc01226b3254</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/faculty/insertPhdDetails?employeeID=64DF070D8B6B11E98B0925647F276BC6&amp;insertData=%7B%22researchTitle%22%3A%22sdfsdf%22%2C%22university%22%3A%22svu%22%2C%22registeredYear%22%3A%222015%22%2C%22phdstatus%22%3A%22Pursuing%22%2C%22dateAward%22%3A%222015-07-05T18%3A30%3A00.000Z%22%2C%22supervisorName%22%3A%22sdfdssdf%22%2C%22supervisionAff%22%3A%22svu%22%2C%22deptId%22%3A%2220196101013404918557388%22%2C%22phdDetailsId%22%3Anull%7D&amp;state=forPhdDetailsInsert</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.URL</defaultValue>
      <description>url</description>
      <id>459690e0-0de0-4f7c-9a7e-68261b53bd28</id>
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
