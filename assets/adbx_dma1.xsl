<?xml version="1.0" encoding="UTF-8"?>
<!-- 
     This is a xsl script to extract birth information in TAB separated lines from an ADB export xml file.
     It is run with the command: xsltproc script name file.xml
     Copyright Alois Treindl, 2016. Published under the GNU Public license version 2 or later.

     Other data can be extracted from theexport xml file in a similar matter.
     xsl is a strange language and needs to be played with.
     Some things are complicated to express in xsl, but it operates powerfully upon xml data.
-->
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:xs="http://www.w3.org/2001/XMLSchema" exclude-result-prefixes="xs">
<xsl:output method="text" encoding="UTF-8" indent="yes"/> 
<xsl:template match="astrodatabank_export">
  <xsl:apply-templates select="adb_entry"/>
</xsl:template>

<xsl:template match="adb_entry">
  <xsl:value-of select="@adb_id"/>
  <!-- tab char -->
  <xsl:text>&#x9;</xsl:text>
  <xsl:apply-templates select="public_data"/>
  <!-- line feed char -->
  <xsl:text>&#10;</xsl:text>
</xsl:template>

<xsl:template match="public_data">
  <xsl:value-of select="name"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="gender"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="roddenrating"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:apply-templates select="bdata"/>
</xsl:template>

<xsl:template match="bdata">
  <xsl:value-of select="sbdate/@ccalendar"/>
  <xsl:value-of select="sbdate"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="sbtime"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="sbtime/@stmerid"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="sbtime/@ctimetype"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="sbtime/@sznabbr"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="sbtime/@jd_ut"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="place"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="country"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="country/@sctr"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="place/@slati"/>
  <xsl:text>&#x9;</xsl:text>
  <xsl:value-of select="place/@slong"/>
</xsl:template>

</xsl:stylesheet>
