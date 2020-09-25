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
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://oecfrontend.vercel.app/')

WebUI.setText(findTestObject('Web/Login/input_Faculty Login_username'), 'DEMO051020')

WebUI.setEncryptedText(findTestObject('Web/Login/input_Faculty Login_password'), 'Sp03FKARx74=')

WebUI.click(findTestObject('Page_/button_Login'))

WebUI.click(findTestObject('Web/Profile/Personal Details/a_Profile'))

WebUI.click(findTestObject('Web/Profile/Qualification Details/button_Qualification Details'))

WebUI.click(findTestObject('Web/Profile/Qualification Details/button_ADD QUALIFICATION'))

WebUI.selectOptionByValue(findTestObject('Web/Profile/Qualification Details/select_Select Qualification Phd'), 'Phd', true)

WebUI.setText(findTestObject('Web/Profile/Qualification Details/input_Add Qualification_Name of The College'), 'JNTU Hyderabad')

WebUI.setText(findTestObject('Web/Profile/Qualification Details/input_Add Qualification_Branch Name'), 'Artificial Intelligence')

WebUI.setText(findTestObject('Web/Profile/Qualification Details/input_Add Qualification_Year of Completion'), '2005')

WebUI.click(findTestObject('Web/Profile/Qualification Details/button_Save Qualification'))

WebUI.click(findTestObject('Web/Profile/Qualification Details/button_Qualification Details'))

WebUI.closeBrowser()

