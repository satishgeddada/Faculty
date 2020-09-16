import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Getting the course coordinator details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Getting the unit topic details onload for syllabus', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the course unit topic details for syllabus', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Uploading the filled syllabus template', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Download the filled syllabus unit topic details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Getting the copo mapping details on load', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Saving single copo item record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Saving the all the copo mapping items at a time', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Getting the exam Question paper entry details onload', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Getting the Question paper entry details if already exist', [
            ('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Saving the question paper entry for the mid-1', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the exam question paper entry details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

