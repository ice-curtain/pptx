mod common;
mod schemas;
mod package;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::read_xml_dcl;
    use crate::schemas::drawing::main::{CtOfficeStyleSheet, CtTableStyleList};
    use crate::schemas::presentation::main::{
        CtCommentAuthorList, CtNotesMaster, CtNotesSlide, CtPresentation, CtPresentationProperties,
        CtSlide, CtSlideLayout, CtSlideMaster, CtTagList, CtViewProperties,
    };
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn it_works() {
        let theme_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
    <p:cSld>
        <p:spTree>
            <p:nvGrpSpPr>
                <p:cNvPr id="1" name=""/>
                <p:cNvGrpSpPr/>
                <p:nvPr/>
            </p:nvGrpSpPr>
            <p:grpSpPr>
                <a:xfrm>
                    <a:off x="0" y="0"/>
                    <a:ext cx="0" cy="0"/>
                    <a:chOff x="0" y="0"/>
                    <a:chExt cx="0" cy="0"/>
                </a:xfrm>
            </p:grpSpPr>
            <p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="48" name="矩形: 圆角 47"/>
                    <p:cNvSpPr/>
                    <p:nvPr/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="-1931667" y="494214"/>
                        <a:ext cx="5047974" cy="361364"/>
                    </a:xfrm>
                    <a:prstGeom prst="roundRect">
                        <a:avLst>
                            <a:gd name="adj" fmla="val 50000"/>
                        </a:avLst>
                    </a:prstGeom>
                    <a:gradFill>
                        <a:gsLst>
                            <a:gs pos="0">
                                <a:srgbClr val="36A19C">
                                    <a:alpha val="0"/>
                                </a:srgbClr>
                            </a:gs>
                            <a:gs pos="100000">
                                <a:srgbClr val="3BD2D1">
                                    <a:alpha val="50000"/>
                                </a:srgbClr>
                            </a:gs>
                        </a:gsLst>
                        <a:lin ang="0" scaled="0"/>
                    </a:gradFill>
                    <a:ln>
                        <a:noFill/>
                    </a:ln>
                </p:spPr>
                <p:style>
                    <a:lnRef idx="2">
                        <a:schemeClr val="accent1">
                            <a:shade val="50000"/>
                        </a:schemeClr>
                    </a:lnRef>
                    <a:fillRef idx="1">
                        <a:schemeClr val="accent1"/>
                    </a:fillRef>
                    <a:effectRef idx="0">
                        <a:schemeClr val="accent1"/>
                    </a:effectRef>
                    <a:fontRef idx="minor">
                        <a:schemeClr val="lt1"/>
                    </a:fontRef>
                </p:style>
                <p:txBody>
                    <a:bodyPr rtlCol="0" anchor="ctr"/>
                    <a:lstStyle>
                        <a:defPPr>
                            <a:defRPr lang="zh-CN"/>
                        </a:defPPr>
                        <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl1pPr>
                        <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl2pPr>
                        <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl3pPr>
                        <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl4pPr>
                        <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl5pPr>
                        <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl6pPr>
                        <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl7pPr>
                        <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl8pPr>
                        <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl9pPr>
                    </a:lstStyle>
                    <a:p>
                        <a:pPr marL="0" marR="0" lvl="0" indent="0" algn="ctr" defTabSz="914400" rtl="0" eaLnBrk="1" fontAlgn="auto" latinLnBrk="0" hangingPunct="1">
                            <a:lnSpc>
                                <a:spcPct val="120000"/>
                            </a:lnSpc>
                            <a:buClrTx/>
                            <a:buSzTx/>
                            <a:buFontTx/>
                            <a:buNone/>
                            <a:defRPr/>
                        </a:pPr>
                        <a:endParaRPr kumimoji="0" lang="zh-CN" altLang="en-US" sz="1800" b="0" i="0" u="none" strike="noStrike" kern="1200" cap="none" spc="0" normalizeH="0" baseline="0" noProof="0">
                            <a:ln>
                                <a:noFill/>
                            </a:ln>
                            <a:solidFill>
                                <a:prstClr val="white"/>
                            </a:solidFill>
                            <a:effectLst/>
                            <a:uLnTx/>
                            <a:uFillTx/>
                            <a:cs typeface="+mn-ea"/>
                            <a:sym typeface="+mn-lt"/>
                        </a:endParaRPr>
                    </a:p>
                </p:txBody>
            </p:sp>
            <p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="49" name="矩形: 圆角 48"/>
                    <p:cNvSpPr/>
                    <p:nvPr/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm rot="10800000">
                        <a:off x="9517143" y="3745160"/>
                        <a:ext cx="4064552" cy="706055"/>
                    </a:xfrm>
                    <a:prstGeom prst="roundRect">
                        <a:avLst>
                            <a:gd name="adj" fmla="val 50000"/>
                        </a:avLst>
                    </a:prstGeom>
                    <a:gradFill>
                        <a:gsLst>
                            <a:gs pos="0">
                                <a:srgbClr val="36A19C">
                                    <a:alpha val="0"/>
                                </a:srgbClr>
                            </a:gs>
                            <a:gs pos="100000">
                                <a:srgbClr val="3BD2D1">
                                    <a:alpha val="50000"/>
                                </a:srgbClr>
                            </a:gs>
                        </a:gsLst>
                        <a:lin ang="0" scaled="0"/>
                    </a:gradFill>
                    <a:ln>
                        <a:noFill/>
                    </a:ln>
                </p:spPr>
                <p:style>
                    <a:lnRef idx="2">
                        <a:schemeClr val="accent1">
                            <a:shade val="50000"/>
                        </a:schemeClr>
                    </a:lnRef>
                    <a:fillRef idx="1">
                        <a:schemeClr val="accent1"/>
                    </a:fillRef>
                    <a:effectRef idx="0">
                        <a:schemeClr val="accent1"/>
                    </a:effectRef>
                    <a:fontRef idx="minor">
                        <a:schemeClr val="lt1"/>
                    </a:fontRef>
                </p:style>
                <p:txBody>
                    <a:bodyPr rtlCol="0" anchor="ctr"/>
                    <a:lstStyle>
                        <a:defPPr>
                            <a:defRPr lang="zh-CN"/>
                        </a:defPPr>
                        <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl1pPr>
                        <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl2pPr>
                        <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl3pPr>
                        <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl4pPr>
                        <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl5pPr>
                        <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl6pPr>
                        <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl7pPr>
                        <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl8pPr>
                        <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl9pPr>
                    </a:lstStyle>
                    <a:p>
                        <a:pPr algn="ctr">
                            <a:lnSpc>
                                <a:spcPct val="120000"/>
                            </a:lnSpc>
                        </a:pPr>
                        <a:endParaRPr lang="zh-CN" altLang="en-US">
                            <a:solidFill>
                                <a:prstClr val="white"/>
                            </a:solidFill>
                            <a:cs typeface="+mn-ea"/>
                            <a:sym typeface="+mn-lt"/>
                        </a:endParaRPr>
                    </a:p>
                </p:txBody>
            </p:sp>
            <p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="51" name="矩形: 圆角 50"/>
                    <p:cNvSpPr/>
                    <p:nvPr/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="-2154154" y="3801055"/>
                        <a:ext cx="6589776" cy="819506"/>
                    </a:xfrm>
                    <a:prstGeom prst="roundRect">
                        <a:avLst>
                            <a:gd name="adj" fmla="val 50000"/>
                        </a:avLst>
                    </a:prstGeom>
                    <a:gradFill>
                        <a:gsLst>
                            <a:gs pos="0">
                                <a:srgbClr val="36A19C">
                                    <a:alpha val="0"/>
                                </a:srgbClr>
                            </a:gs>
                            <a:gs pos="100000">
                                <a:srgbClr val="3BD2D1">
                                    <a:alpha val="40000"/>
                                </a:srgbClr>
                            </a:gs>
                        </a:gsLst>
                        <a:lin ang="0" scaled="0"/>
                    </a:gradFill>
                    <a:ln>
                        <a:noFill/>
                    </a:ln>
                </p:spPr>
                <p:style>
                    <a:lnRef idx="2">
                        <a:schemeClr val="accent1">
                            <a:shade val="50000"/>
                        </a:schemeClr>
                    </a:lnRef>
                    <a:fillRef idx="1">
                        <a:schemeClr val="accent1"/>
                    </a:fillRef>
                    <a:effectRef idx="0">
                        <a:schemeClr val="accent1"/>
                    </a:effectRef>
                    <a:fontRef idx="minor">
                        <a:schemeClr val="lt1"/>
                    </a:fontRef>
                </p:style>
                <p:txBody>
                    <a:bodyPr rtlCol="0" anchor="ctr"/>
                    <a:lstStyle>
                        <a:defPPr>
                            <a:defRPr lang="zh-CN"/>
                        </a:defPPr>
                        <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl1pPr>
                        <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl2pPr>
                        <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl3pPr>
                        <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl4pPr>
                        <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl5pPr>
                        <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl6pPr>
                        <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl7pPr>
                        <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl8pPr>
                        <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl9pPr>
                    </a:lstStyle>
                    <a:p>
                        <a:pPr algn="ctr">
                            <a:lnSpc>
                                <a:spcPct val="120000"/>
                            </a:lnSpc>
                        </a:pPr>
                        <a:endParaRPr lang="zh-CN" altLang="en-US">
                            <a:solidFill>
                                <a:prstClr val="white"/>
                            </a:solidFill>
                            <a:cs typeface="+mn-ea"/>
                            <a:sym typeface="+mn-lt"/>
                        </a:endParaRPr>
                    </a:p>
                </p:txBody>
            </p:sp>
            <p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="52" name="矩形: 圆角 51"/>
                    <p:cNvSpPr/>
                    <p:nvPr/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm rot="10800000">
                        <a:off x="7756377" y="5755241"/>
                        <a:ext cx="6589776" cy="608546"/>
                    </a:xfrm>
                    <a:prstGeom prst="roundRect">
                        <a:avLst>
                            <a:gd name="adj" fmla="val 50000"/>
                        </a:avLst>
                    </a:prstGeom>
                    <a:gradFill>
                        <a:gsLst>
                            <a:gs pos="0">
                                <a:srgbClr val="36A19C">
                                    <a:alpha val="0"/>
                                </a:srgbClr>
                            </a:gs>
                            <a:gs pos="100000">
                                <a:srgbClr val="3BD2D1">
                                    <a:alpha val="40000"/>
                                </a:srgbClr>
                            </a:gs>
                        </a:gsLst>
                        <a:lin ang="0" scaled="0"/>
                    </a:gradFill>
                    <a:ln>
                        <a:noFill/>
                    </a:ln>
                </p:spPr>
                <p:style>
                    <a:lnRef idx="2">
                        <a:schemeClr val="accent1">
                            <a:shade val="50000"/>
                        </a:schemeClr>
                    </a:lnRef>
                    <a:fillRef idx="1">
                        <a:schemeClr val="accent1"/>
                    </a:fillRef>
                    <a:effectRef idx="0">
                        <a:schemeClr val="accent1"/>
                    </a:effectRef>
                    <a:fontRef idx="minor">
                        <a:schemeClr val="lt1"/>
                    </a:fontRef>
                </p:style>
                <p:txBody>
                    <a:bodyPr rtlCol="0" anchor="ctr"/>
                    <a:lstStyle>
                        <a:defPPr>
                            <a:defRPr lang="zh-CN"/>
                        </a:defPPr>
                        <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl1pPr>
                        <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl2pPr>
                        <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl3pPr>
                        <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl4pPr>
                        <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl5pPr>
                        <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl6pPr>
                        <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl7pPr>
                        <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl8pPr>
                        <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl9pPr>
                    </a:lstStyle>
                    <a:p>
                        <a:pPr algn="ctr">
                            <a:lnSpc>
                                <a:spcPct val="120000"/>
                            </a:lnSpc>
                        </a:pPr>
                        <a:endParaRPr lang="zh-CN" altLang="en-US">
                            <a:solidFill>
                                <a:prstClr val="white"/>
                            </a:solidFill>
                            <a:cs typeface="+mn-ea"/>
                            <a:sym typeface="+mn-lt"/>
                        </a:endParaRPr>
                    </a:p>
                </p:txBody>
            </p:sp>
            <p:pic>
                <p:nvPicPr>
                    <p:cNvPr id="32" name="Picture 7"/>
                    <p:cNvPicPr>
                        <a:picLocks noChangeAspect="1"/>
                    </p:cNvPicPr>
                    <p:nvPr/>
                </p:nvPicPr>
                <p:blipFill>
                    <a:blip r:embed="rId3">
                        <a:clrChange>
                            <a:clrFrom>
                                <a:srgbClr val="FFFFFF"/>
                            </a:clrFrom>
                            <a:clrTo>
                                <a:srgbClr val="FFFFFF">
                                    <a:alpha val="0"/>
                                </a:srgbClr>
                            </a:clrTo>
                        </a:clrChange>
                    </a:blip>
                    <a:stretch>
                        <a:fillRect/>
                    </a:stretch>
                </p:blipFill>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="10081608" y="149729"/>
                        <a:ext cx="2033098" cy="578455"/>
                    </a:xfrm>
                    <a:prstGeom prst="rect">
                        <a:avLst/>
                    </a:prstGeom>
                </p:spPr>
            </p:pic>
            <p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="46" name="矩形: 圆角 45"/>
                    <p:cNvSpPr/>
                    <p:nvPr/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="-598048" y="3027236"/>
                        <a:ext cx="8711950" cy="1991253"/>
                    </a:xfrm>
                    <a:prstGeom prst="roundRect">
                        <a:avLst>
                            <a:gd name="adj" fmla="val 50000"/>
                        </a:avLst>
                    </a:prstGeom>
                    <a:gradFill>
                        <a:gsLst>
                            <a:gs pos="0">
                                <a:srgbClr val="3BD2D1">
                                    <a:alpha val="2000"/>
                                </a:srgbClr>
                            </a:gs>
                            <a:gs pos="100000">
                                <a:srgbClr val="36A19C">
                                    <a:alpha val="10000"/>
                                </a:srgbClr>
                            </a:gs>
                        </a:gsLst>
                        <a:lin ang="0" scaled="0"/>
                    </a:gradFill>
                    <a:ln>
                        <a:noFill/>
                    </a:ln>
                </p:spPr>
                <p:style>
                    <a:lnRef idx="2">
                        <a:schemeClr val="accent1">
                            <a:shade val="50000"/>
                        </a:schemeClr>
                    </a:lnRef>
                    <a:fillRef idx="1">
                        <a:schemeClr val="accent1"/>
                    </a:fillRef>
                    <a:effectRef idx="0">
                        <a:schemeClr val="accent1"/>
                    </a:effectRef>
                    <a:fontRef idx="minor">
                        <a:schemeClr val="lt1"/>
                    </a:fontRef>
                </p:style>
                <p:txBody>
                    <a:bodyPr rtlCol="0" anchor="ctr"/>
                    <a:lstStyle>
                        <a:defPPr>
                            <a:defRPr lang="zh-CN"/>
                        </a:defPPr>
                        <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl1pPr>
                        <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl2pPr>
                        <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl3pPr>
                        <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl4pPr>
                        <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl5pPr>
                        <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl6pPr>
                        <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl7pPr>
                        <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl8pPr>
                        <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl9pPr>
                    </a:lstStyle>
                    <a:p>
                        <a:pPr algn="ctr">
                            <a:lnSpc>
                                <a:spcPct val="120000"/>
                            </a:lnSpc>
                        </a:pPr>
                        <a:endParaRPr lang="zh-CN" altLang="en-US">
                            <a:solidFill>
                                <a:prstClr val="white"/>
                            </a:solidFill>
                            <a:cs typeface="+mn-ea"/>
                            <a:sym typeface="+mn-lt"/>
                        </a:endParaRPr>
                    </a:p>
                </p:txBody>
            </p:sp>
            <p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="47" name="矩形: 圆角 46"/>
                    <p:cNvSpPr/>
                    <p:nvPr/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm rot="10800000">
                        <a:off x="4078099" y="1839511"/>
                        <a:ext cx="8711950" cy="1991253"/>
                    </a:xfrm>
                    <a:prstGeom prst="roundRect">
                        <a:avLst>
                            <a:gd name="adj" fmla="val 50000"/>
                        </a:avLst>
                    </a:prstGeom>
                    <a:gradFill>
                        <a:gsLst>
                            <a:gs pos="0">
                                <a:srgbClr val="3BD2D1">
                                    <a:alpha val="2000"/>
                                </a:srgbClr>
                            </a:gs>
                            <a:gs pos="100000">
                                <a:srgbClr val="36A19C">
                                    <a:alpha val="10000"/>
                                </a:srgbClr>
                            </a:gs>
                        </a:gsLst>
                        <a:lin ang="0" scaled="0"/>
                    </a:gradFill>
                    <a:ln>
                        <a:noFill/>
                    </a:ln>
                </p:spPr>
                <p:style>
                    <a:lnRef idx="2">
                        <a:schemeClr val="accent1">
                            <a:shade val="50000"/>
                        </a:schemeClr>
                    </a:lnRef>
                    <a:fillRef idx="1">
                        <a:schemeClr val="accent1"/>
                    </a:fillRef>
                    <a:effectRef idx="0">
                        <a:schemeClr val="accent1"/>
                    </a:effectRef>
                    <a:fontRef idx="minor">
                        <a:schemeClr val="lt1"/>
                    </a:fontRef>
                </p:style>
                <p:txBody>
                    <a:bodyPr rtlCol="0" anchor="ctr"/>
                    <a:lstStyle>
                        <a:defPPr>
                            <a:defRPr lang="zh-CN"/>
                        </a:defPPr>
                        <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl1pPr>
                        <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl2pPr>
                        <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl3pPr>
                        <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl4pPr>
                        <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl5pPr>
                        <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl6pPr>
                        <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl7pPr>
                        <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl8pPr>
                        <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                            <a:defRPr sz="1800" kern="1200">
                                <a:solidFill>
                                    <a:schemeClr val="lt1"/>
                                </a:solidFill>
                                <a:latin typeface="+mn-lt"/>
                                <a:ea typeface="+mn-ea"/>
                                <a:cs typeface="+mn-cs"/>
                            </a:defRPr>
                        </a:lvl9pPr>
                    </a:lstStyle>
                    <a:p>
                        <a:pPr marL="0" marR="0" lvl="0" indent="0" algn="ctr" defTabSz="914400" rtl="0" eaLnBrk="1" fontAlgn="auto" latinLnBrk="0" hangingPunct="1">
                            <a:lnSpc>
                                <a:spcPct val="120000"/>
                            </a:lnSpc>
                            <a:buClrTx/>
                            <a:buSzTx/>
                            <a:buFontTx/>
                            <a:buNone/>
                            <a:defRPr/>
                        </a:pPr>
                        <a:endParaRPr kumimoji="0" lang="zh-CN" altLang="en-US" sz="1800" b="0" i="0" u="none" strike="noStrike" kern="1200" cap="none" spc="0" normalizeH="0" baseline="0" noProof="0">
                            <a:ln>
                                <a:noFill/>
                            </a:ln>
                            <a:solidFill>
                                <a:prstClr val="white"/>
                            </a:solidFill>
                            <a:effectLst/>
                            <a:uLnTx/>
                            <a:uFillTx/>
                            <a:cs typeface="+mn-ea"/>
                            <a:sym typeface="+mn-lt"/>
                        </a:endParaRPr>
                    </a:p>
                </p:txBody>
            </p:sp>
            <p:grpSp>
                <p:nvGrpSpPr>
                    <p:cNvPr id="55" name="组合 54"/>
                    <p:cNvGrpSpPr/>
                    <p:nvPr/>
                </p:nvGrpSpPr>
                <p:grpSpPr>
                    <a:xfrm>
                        <a:off x="1135766" y="2215559"/>
                        <a:ext cx="9962391" cy="2961839"/>
                        <a:chOff x="1135766" y="2215559"/>
                        <a:chExt cx="9962391" cy="2961839"/>
                    </a:xfrm>
                </p:grpSpPr>
                <p:sp>
                    <p:nvSpPr>
                        <p:cNvPr id="7" name="文本框 1"/>
                        <p:cNvSpPr txBox="1">
                            <a:spLocks noChangeArrowheads="1"/>
                        </p:cNvSpPr>
                        <p:nvPr/>
                    </p:nvSpPr>
                    <p:spPr bwMode="auto">
                        <a:xfrm>
                            <a:off x="4422702" y="3229480"/>
                            <a:ext cx="3346596" cy="396583"/>
                        </a:xfrm>
                        <a:prstGeom prst="rect">
                            <a:avLst/>
                        </a:prstGeom>
                        <a:noFill/>
                        <a:ln>
                            <a:noFill/>
                        </a:ln>
                        <a:extLst>
                            <a:ext uri="{909E8E84-426E-40DD-AFC4-6F175D3DCCD1}">
                                <a14:hiddenFill xmlns:a14="http://schemas.microsoft.com/office/drawing/2010/main">
                                    <a:solidFill>
                                        <a:srgbClr val="FFFFFF"/>
                                    </a:solidFill>
                                </a14:hiddenFill>
                            </a:ext>
                            <a:ext uri="{91240B29-F687-4F45-9708-019B960494DF}">
                                <a14:hiddenLine xmlns:a14="http://schemas.microsoft.com/office/drawing/2010/main" w="9525">
                                    <a:solidFill>
                                        <a:srgbClr val="000000"/>
                                    </a:solidFill>
                                    <a:miter lim="800000"/>
                                    <a:headEnd/>
                                    <a:tailEnd/>
                                </a14:hiddenLine>
                            </a:ext>
                        </a:extLst>
                    </p:spPr>
                    <p:txBody>
                        <a:bodyPr wrap="square">
                            <a:spAutoFit/>
                        </a:bodyPr>
                        <a:lstStyle>
                            <a:defPPr>
                                <a:defRPr lang="zh-CN"/>
                            </a:defPPr>
                            <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl1pPr>
                            <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl2pPr>
                            <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl3pPr>
                            <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl4pPr>
                            <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl5pPr>
                            <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl6pPr>
                            <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl7pPr>
                            <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl8pPr>
                            <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl9pPr>
                        </a:lstStyle>
                        <a:p>
                            <a:pPr algn="ctr" defTabSz="685165">
                                <a:lnSpc>
                                    <a:spcPct val="120000"/>
                                </a:lnSpc>
                                <a:defRPr/>
                            </a:pPr>
                            <a:r>
                                <a:rPr lang="zh-CN" altLang="en-US" noProof="1">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1">
                                            <a:lumMod val="65000"/>
                                            <a:lumOff val="35000"/>
                                        </a:schemeClr>
                                    </a:solidFill>
                                    <a:cs typeface="+mn-ea"/>
                                    <a:sym typeface="+mn-lt"/>
                                </a:rPr>
                                <a:t>上海零假设信息科技有限公司</a:t>
                            </a:r>
                        </a:p>
                    </p:txBody>
                </p:sp>
                <p:sp>
                    <p:nvSpPr>
                        <p:cNvPr id="35" name="文本框 1"/>
                        <p:cNvSpPr txBox="1">
                            <a:spLocks noChangeArrowheads="1"/>
                        </p:cNvSpPr>
                        <p:nvPr/>
                    </p:nvSpPr>
                    <p:spPr bwMode="auto">
                        <a:xfrm>
                            <a:off x="3406238" y="4780815"/>
                            <a:ext cx="5573236" cy="396583"/>
                        </a:xfrm>
                        <a:prstGeom prst="rect">
                            <a:avLst/>
                        </a:prstGeom>
                        <a:noFill/>
                        <a:ln>
                            <a:noFill/>
                        </a:ln>
                        <a:extLst>
                            <a:ext uri="{909E8E84-426E-40DD-AFC4-6F175D3DCCD1}">
                                <a14:hiddenFill xmlns:a14="http://schemas.microsoft.com/office/drawing/2010/main">
                                    <a:solidFill>
                                        <a:srgbClr val="FFFFFF"/>
                                    </a:solidFill>
                                </a14:hiddenFill>
                            </a:ext>
                            <a:ext uri="{91240B29-F687-4F45-9708-019B960494DF}">
                                <a14:hiddenLine xmlns:a14="http://schemas.microsoft.com/office/drawing/2010/main" w="9525">
                                    <a:solidFill>
                                        <a:srgbClr val="000000"/>
                                    </a:solidFill>
                                    <a:miter lim="800000"/>
                                    <a:headEnd/>
                                    <a:tailEnd/>
                                </a14:hiddenLine>
                            </a:ext>
                        </a:extLst>
                    </p:spPr>
                    <p:txBody>
                        <a:bodyPr wrap="square">
                            <a:spAutoFit/>
                        </a:bodyPr>
                        <a:lstStyle>
                            <a:defPPr>
                                <a:defRPr lang="zh-CN"/>
                            </a:defPPr>
                            <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl1pPr>
                            <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl2pPr>
                            <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl3pPr>
                            <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl4pPr>
                            <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl5pPr>
                            <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl6pPr>
                            <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl7pPr>
                            <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl8pPr>
                            <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                <a:defRPr sz="1800" kern="1200">
                                    <a:solidFill>
                                        <a:schemeClr val="tx1"/>
                                    </a:solidFill>
                                    <a:latin typeface="+mn-lt"/>
                                    <a:ea typeface="+mn-ea"/>
                                    <a:cs typeface="+mn-cs"/>
                                </a:defRPr>
                            </a:lvl9pPr>
                        </a:lstStyle>
                        <a:p>
                            <a:pPr marL="0" marR="0" lvl="0" indent="0" algn="ctr" defTabSz="685165" rtl="0" eaLnBrk="1" fontAlgn="auto" latinLnBrk="0" hangingPunct="1">
                                <a:lnSpc>
                                    <a:spcPct val="120000"/>
                                </a:lnSpc>
                                <a:buClrTx/>
                                <a:buSzTx/>
                                <a:buFontTx/>
                                <a:buNone/>
                                <a:defRPr/>
                            </a:pPr>
                            <a:r>
                                <a:rPr lang="zh-CN" altLang="en-US" b="1" noProof="1">
                                    <a:solidFill>
                                        <a:srgbClr val="36A19C"/>
                                    </a:solidFill>
                                    <a:cs typeface="+mn-ea"/>
                                    <a:sym typeface="+mn-lt"/>
                                </a:rPr>
                                <a:t>文章排列顺序按 </a:t>
                            </a:r>
                            <a:r>
                                <a:rPr lang="en-US" altLang="zh-CN" b="1" noProof="1">
                                    <a:solidFill>
                                        <a:srgbClr val="36A19C"/>
                                    </a:solidFill>
                                    <a:cs typeface="+mn-ea"/>
                                    <a:sym typeface="+mn-lt"/>
                                </a:rPr>
                                <a:t>publication</a:t>
                            </a:r>
                            <a:r>
                                <a:rPr lang="zh-CN" altLang="en-US" b="1" noProof="1">
                                    <a:solidFill>
                                        <a:srgbClr val="36A19C"/>
                                    </a:solidFill>
                                    <a:cs typeface="+mn-ea"/>
                                    <a:sym typeface="+mn-lt"/>
                                </a:rPr>
                                <a:t>
                                </a:t>
                            </a:r>
                            <a:r>
                                <a:rPr lang="en-US" altLang="zh-CN" b="1" noProof="1">
                                    <a:solidFill>
                                        <a:srgbClr val="36A19C"/>
                                    </a:solidFill>
                                    <a:cs typeface="+mn-ea"/>
                                    <a:sym typeface="+mn-lt"/>
                                </a:rPr>
                                <a:t>date</a:t>
                            </a:r>
                            <a:r>
                                <a:rPr lang="zh-CN" altLang="en-US" b="1" noProof="1">
                                    <a:solidFill>
                                        <a:srgbClr val="36A19C"/>
                                    </a:solidFill>
                                    <a:cs typeface="+mn-ea"/>
                                    <a:sym typeface="+mn-lt"/>
                                </a:rPr>
                                <a:t> 由近至远排列</a:t>
                            </a:r>
                            <a:endParaRPr kumimoji="0" lang="zh-CN" altLang="en-US" b="0" i="0" u="none" strike="noStrike" kern="1200" cap="none" spc="0" normalizeH="0" baseline="0" noProof="1">
                                <a:ln>
                                    <a:noFill/>
                                </a:ln>
                                <a:solidFill>
                                    <a:srgbClr val="36A19C"/>
                                </a:solidFill>
                                <a:effectLst/>
                                <a:uLnTx/>
                                <a:uFillTx/>
                                <a:cs typeface="+mn-ea"/>
                                <a:sym typeface="+mn-lt"/>
                            </a:endParaRPr>
                        </a:p>
                    </p:txBody>
                </p:sp>
                <p:grpSp>
                    <p:nvGrpSpPr>
                        <p:cNvPr id="54" name="组合 53"/>
                        <p:cNvGrpSpPr/>
                        <p:nvPr/>
                    </p:nvGrpSpPr>
                    <p:grpSpPr>
                        <a:xfrm>
                            <a:off x="1135766" y="2215559"/>
                            <a:ext cx="9962391" cy="1106393"/>
                            <a:chOff x="1135766" y="2215559"/>
                            <a:chExt cx="9962391" cy="1106393"/>
                        </a:xfrm>
                    </p:grpSpPr>
                    <p:sp>
                        <p:nvSpPr>
                            <p:cNvPr id="50" name="矩形: 圆角 49"/>
                            <p:cNvSpPr/>
                            <p:nvPr/>
                        </p:nvSpPr>
                        <p:spPr>
                            <a:xfrm rot="10800000">
                                <a:off x="10139075" y="2694459"/>
                                <a:ext cx="959082" cy="57864"/>
                            </a:xfrm>
                            <a:prstGeom prst="roundRect">
                                <a:avLst>
                                    <a:gd name="adj" fmla="val 50000"/>
                                </a:avLst>
                            </a:prstGeom>
                            <a:gradFill>
                                <a:gsLst>
                                    <a:gs pos="0">
                                        <a:srgbClr val="36A19C">
                                            <a:alpha val="0"/>
                                        </a:srgbClr>
                                    </a:gs>
                                    <a:gs pos="100000">
                                        <a:srgbClr val="3BD2D1"/>
                                    </a:gs>
                                </a:gsLst>
                                <a:lin ang="0" scaled="0"/>
                            </a:gradFill>
                            <a:ln>
                                <a:noFill/>
                            </a:ln>
                        </p:spPr>
                        <p:style>
                            <a:lnRef idx="2">
                                <a:schemeClr val="accent1">
                                    <a:shade val="50000"/>
                                </a:schemeClr>
                            </a:lnRef>
                            <a:fillRef idx="1">
                                <a:schemeClr val="accent1"/>
                            </a:fillRef>
                            <a:effectRef idx="0">
                                <a:schemeClr val="accent1"/>
                            </a:effectRef>
                            <a:fontRef idx="minor">
                                <a:schemeClr val="lt1"/>
                            </a:fontRef>
                        </p:style>
                        <p:txBody>
                            <a:bodyPr rtlCol="0" anchor="ctr"/>
                            <a:lstStyle>
                                <a:defPPr>
                                    <a:defRPr lang="zh-CN"/>
                                </a:defPPr>
                                <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl1pPr>
                                <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl2pPr>
                                <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl3pPr>
                                <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl4pPr>
                                <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl5pPr>
                                <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl6pPr>
                                <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl7pPr>
                                <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl8pPr>
                                <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl9pPr>
                            </a:lstStyle>
                            <a:p>
                                <a:pPr marL="0" marR="0" lvl="0" indent="0" algn="ctr" defTabSz="914400" rtl="0" eaLnBrk="1" fontAlgn="auto" latinLnBrk="0" hangingPunct="1">
                                    <a:lnSpc>
                                        <a:spcPct val="120000"/>
                                    </a:lnSpc>
                                    <a:buClrTx/>
                                    <a:buSzTx/>
                                    <a:buFontTx/>
                                    <a:buNone/>
                                    <a:defRPr/>
                                </a:pPr>
                                <a:endParaRPr kumimoji="0" lang="zh-CN" altLang="en-US" sz="1800" b="0" i="0" u="none" strike="noStrike" kern="1200" cap="none" spc="0" normalizeH="0" baseline="0" noProof="0">
                                    <a:ln>
                                        <a:noFill/>
                                    </a:ln>
                                    <a:solidFill>
                                        <a:prstClr val="white"/>
                                    </a:solidFill>
                                    <a:effectLst/>
                                    <a:uLnTx/>
                                    <a:uFillTx/>
                                    <a:cs typeface="+mn-ea"/>
                                    <a:sym typeface="+mn-lt"/>
                                </a:endParaRPr>
                            </a:p>
                        </p:txBody>
                    </p:sp>
                    <p:sp>
                        <p:nvSpPr>
                            <p:cNvPr id="8" name="文本框 2"/>
                            <p:cNvSpPr txBox="1"/>
                            <p:nvPr/>
                        </p:nvSpPr>
                        <p:spPr>
                            <a:xfrm>
                                <a:off x="1758462" y="2215559"/>
                                <a:ext cx="8675076" cy="1106393"/>
                            </a:xfrm>
                            <a:prstGeom prst="rect">
                                <a:avLst/>
                            </a:prstGeom>
                            <a:noFill/>
                        </p:spPr>
                        <p:txBody>
                            <a:bodyPr wrap="square">
                                <a:spAutoFit/>
                            </a:bodyPr>
                            <a:lstStyle>
                                <a:defPPr>
                                    <a:defRPr lang="zh-CN"/>
                                </a:defPPr>
                                <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="tx1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl1pPr>
                                <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="tx1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl2pPr>
                                <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="tx1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl3pPr>
                                <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="tx1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl4pPr>
                                <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="tx1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl5pPr>
                                <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="tx1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl6pPr>
                                <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="tx1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl7pPr>
                                <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="tx1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl8pPr>
                                <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="tx1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl9pPr>
                            </a:lstStyle>
                            <a:p>
                                <a:pPr marL="0" marR="0" lvl="0" indent="0" algn="ctr" defTabSz="685165" rtl="0" eaLnBrk="1" fontAlgn="auto" latinLnBrk="0" hangingPunct="1">
                                    <a:lnSpc>
                                        <a:spcPct val="120000"/>
                                    </a:lnSpc>
                                    <a:buClrTx/>
                                    <a:buSzTx/>
                                    <a:buFontTx/>
                                    <a:buNone/>
                                    <a:defRPr/>
                                </a:pPr>
                                <a:r>
                                    <a:rPr kumimoji="0" lang="zh-CN" altLang="en-US" sz="6000" b="1" i="0" u="none" strike="noStrike" kern="1200" cap="none" spc="0" normalizeH="0" baseline="0" noProof="1">
                                        <a:ln>
                                            <a:noFill/>
                                        </a:ln>
                                        <a:gradFill flip="none" rotWithShape="1">
                                            <a:gsLst>
                                                <a:gs pos="0">
                                                    <a:srgbClr val="36A19C"/>
                                                </a:gs>
                                                <a:gs pos="100000">
                                                    <a:srgbClr val="3BD2D1"/>
                                                </a:gs>
                                            </a:gsLst>
                                            <a:lin ang="16200000" scaled="1"/>
                                            <a:tileRect/>
                                        </a:gradFill>
                                        <a:effectLst>
                                            <a:outerShdw blurRad="50800" dist="38100" dir="2700000" algn="tl" rotWithShape="0">
                                                <a:prstClr val="black">
                                                    <a:alpha val="20000"/>
                                                </a:prstClr>
                                            </a:outerShdw>
                                        </a:effectLst>
                                        <a:uLnTx/>
                                        <a:uFillTx/>
                                        <a:cs typeface="+mn-ea"/>
                                        <a:sym typeface="+mn-lt"/>
                                    </a:rPr>
                                    <a:t>胰腺癌月度追踪报告</a:t>
                                </a:r>
                                <a:endParaRPr kumimoji="0" lang="en-US" altLang="zh-CN" sz="6000" b="1" i="0" u="none" strike="noStrike" kern="1200" cap="none" spc="0" normalizeH="0" baseline="0" noProof="1">
                                    <a:ln>
                                        <a:noFill/>
                                    </a:ln>
                                    <a:gradFill flip="none" rotWithShape="1">
                                        <a:gsLst>
                                            <a:gs pos="0">
                                                <a:srgbClr val="36A19C"/>
                                            </a:gs>
                                            <a:gs pos="100000">
                                                <a:srgbClr val="3BD2D1"/>
                                            </a:gs>
                                        </a:gsLst>
                                        <a:lin ang="16200000" scaled="1"/>
                                        <a:tileRect/>
                                    </a:gradFill>
                                    <a:effectLst>
                                        <a:outerShdw blurRad="50800" dist="38100" dir="2700000" algn="tl" rotWithShape="0">
                                            <a:prstClr val="black">
                                                <a:alpha val="20000"/>
                                            </a:prstClr>
                                        </a:outerShdw>
                                    </a:effectLst>
                                    <a:uLnTx/>
                                    <a:uFillTx/>
                                    <a:cs typeface="+mn-ea"/>
                                    <a:sym typeface="+mn-lt"/>
                                </a:endParaRPr>
                            </a:p>
                        </p:txBody>
                    </p:sp>
                    <p:sp>
                        <p:nvSpPr>
                            <p:cNvPr id="53" name="矩形: 圆角 52"/>
                            <p:cNvSpPr/>
                            <p:nvPr/>
                        </p:nvSpPr>
                        <p:spPr>
                            <a:xfrm rot="10800000" flipH="1">
                                <a:off x="1135766" y="2694458"/>
                                <a:ext cx="959082" cy="57864"/>
                            </a:xfrm>
                            <a:prstGeom prst="roundRect">
                                <a:avLst>
                                    <a:gd name="adj" fmla="val 50000"/>
                                </a:avLst>
                            </a:prstGeom>
                            <a:gradFill>
                                <a:gsLst>
                                    <a:gs pos="0">
                                        <a:srgbClr val="36A19C">
                                            <a:alpha val="0"/>
                                        </a:srgbClr>
                                    </a:gs>
                                    <a:gs pos="100000">
                                        <a:srgbClr val="3BD2D1"/>
                                    </a:gs>
                                </a:gsLst>
                                <a:lin ang="0" scaled="0"/>
                            </a:gradFill>
                            <a:ln>
                                <a:noFill/>
                            </a:ln>
                        </p:spPr>
                        <p:style>
                            <a:lnRef idx="2">
                                <a:schemeClr val="accent1">
                                    <a:shade val="50000"/>
                                </a:schemeClr>
                            </a:lnRef>
                            <a:fillRef idx="1">
                                <a:schemeClr val="accent1"/>
                            </a:fillRef>
                            <a:effectRef idx="0">
                                <a:schemeClr val="accent1"/>
                            </a:effectRef>
                            <a:fontRef idx="minor">
                                <a:schemeClr val="lt1"/>
                            </a:fontRef>
                        </p:style>
                        <p:txBody>
                            <a:bodyPr rtlCol="0" anchor="ctr"/>
                            <a:lstStyle>
                                <a:defPPr>
                                    <a:defRPr lang="zh-CN"/>
                                </a:defPPr>
                                <a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl1pPr>
                                <a:lvl2pPr marL="457200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl2pPr>
                                <a:lvl3pPr marL="914400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl3pPr>
                                <a:lvl4pPr marL="1371600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl4pPr>
                                <a:lvl5pPr marL="1828800" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl5pPr>
                                <a:lvl6pPr marL="2286000" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl6pPr>
                                <a:lvl7pPr marL="2743200" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl7pPr>
                                <a:lvl8pPr marL="3200400" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl8pPr>
                                <a:lvl9pPr marL="3657600" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
                                    <a:defRPr sz="1800" kern="1200">
                                        <a:solidFill>
                                            <a:schemeClr val="lt1"/>
                                        </a:solidFill>
                                        <a:latin typeface="+mn-lt"/>
                                        <a:ea typeface="+mn-ea"/>
                                        <a:cs typeface="+mn-cs"/>
                                    </a:defRPr>
                                </a:lvl9pPr>
                            </a:lstStyle>
                            <a:p>
                                <a:pPr algn="ctr">
                                    <a:lnSpc>
                                        <a:spcPct val="120000"/>
                                    </a:lnSpc>
                                </a:pPr>
                                <a:endParaRPr lang="zh-CN" altLang="en-US">
                                    <a:solidFill>
                                        <a:prstClr val="white"/>
                                    </a:solidFill>
                                    <a:cs typeface="+mn-ea"/>
                                    <a:sym typeface="+mn-lt"/>
                                </a:endParaRPr>
                            </a:p>
                        </p:txBody>
                    </p:sp>
                </p:grpSp>
            </p:grpSp>
            <p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="2" name="Rectangle 1"/>
                    <p:cNvSpPr/>
                    <p:nvPr/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="5542802" y="4079356"/>
                        <a:ext cx="1106393" cy="396583"/>
                    </a:xfrm>
                    <a:prstGeom prst="rect">
                        <a:avLst/>
                    </a:prstGeom>
                </p:spPr>
                <p:txBody>
                    <a:bodyPr wrap="none">
                        <a:spAutoFit/>
                    </a:bodyPr>
                    <a:lstStyle/>
                    <a:p>
                        <a:pPr lvl="0" algn="ctr" defTabSz="685165">
                            <a:lnSpc>
                                <a:spcPct val="120000"/>
                            </a:lnSpc>
                            <a:defRPr/>
                        </a:pPr>
                        <a:r>
                            <a:rPr lang="en-US" altLang="zh-CN" b="1" noProof="1">
                                <a:solidFill>
                                    <a:srgbClr val="36A19C"/>
                                </a:solidFill>
                                <a:cs typeface="+mn-ea"/>
                                <a:sym typeface="+mn-lt"/>
                            </a:rPr>
                            <a:t>2022.02</a:t>
                        </a:r>
                    </a:p>
                </p:txBody>
            </p:sp>
        </p:spTree>
    </p:cSld>
    <p:clrMapOvr>
        <a:masterClrMapping/>
    </p:clrMapOvr>
</p:sld>"#;
        let theme: CtSlide = quick_xml::de::from_str(theme_xml).unwrap();
        // println!("{:?}",theme);
        // println!("{:?}",quick_xml::se::to_string(&theme));
        let mut file = File::create("/Users/kevin/Documents/ECMA-376-1_5th_edition_december_2016/OfficeOpenXML-XMLSchema-Strict/dist.xml").unwrap();

        let mut buffer = String::new();

        quick_xml::se::to_writer(&mut buffer, &theme).unwrap();

        let dcl = read_xml_dcl(&theme_xml);
        if let Ok(dcl) = dcl {
            let _ = file.write(dcl.as_bytes());
        }
        let result = file.write(buffer.as_ref());
        println!("{:?}", result);
        // quick_xml::se::to_writer(file,&theme);
    }
}
