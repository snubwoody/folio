pragma Singleton
import QtQuick 6.10

QtObject {
    readonly property color white: "#FFFFFF"
    readonly property color black: "#000000"

    // Neutral
    readonly property color neutral25: "#FCFCFC"
    readonly property color neutral50: "#F7F7F7"
    readonly property color neutral100: "#E6E6E6"
    readonly property color neutral200: "#CBCBCB"
    readonly property color neutral300: "#BEBEBE"
    readonly property color neutral400: "#9E9E9E"
    readonly property color neutral500: "#808080"
    readonly property color neutral600: "#787878"
    readonly property color neutral700: "#626262"
    readonly property color neutral800: "#3E3E3E"
    readonly property color neutral900: "#282828"
    readonly property color neutral950: "#171717"

    // Purple
    readonly property color purple50: "#EBE9FE"
    readonly property color purple100: "#D9D6FD"
    readonly property color purple200: "#B2AAFB"
    readonly property color purple300: "#8F80F9"
    readonly property color purple400: "#6D52F6"
    readonly property color purple500: "#4B19E2"
    readonly property color purple600: "#3D13BC"
    readonly property color purple700: "#2E0C94"
    readonly property color purple800: "#20076E"
    readonly property color purple900: "#14034E"
    readonly property color purple950: "#0A0133"

    // Blue
    readonly property color blue50: "#E5F2FF"
    readonly property color blue100: "#CEE9FF"
    readonly property color blue200: "#95D3FF"
    readonly property color blue300: "#32BFFF"
    readonly property color blue400: "#03A5E0"
    readonly property color blue500: "#0088B9"
    readonly property color blue600: "#006B92"
    readonly property color blue700: "#015170"
    readonly property color blue800: "#00374D"
    readonly property color blue900: "#002030"
    readonly property color blue950: "#00131E"

    // Red
    readonly property color red50: "#FEEDED"
    readonly property color red100: "#FEDADA"
    readonly property color red200: "#FDB4B4"
    readonly property color red300: "#FC8A8B"
    readonly property color red400: "#FB5758"
    readonly property color red500: "#EC181C"
    readonly property color red600: "#BD1013"
    readonly property color red700: "#910B0C"
    readonly property color red800: "#670406"
    readonly property color red900: "#400202"
    readonly property color red950: "#2C0101"

    // Border colors
    readonly property color borderNeutral: neutral50
    readonly property color borderFocus: purple500

    // Text colors
    readonly property color textHeading: neutral950
    readonly property color textBody: neutral900
    readonly property color textMuted: neutral600
    readonly property color textError: red400

    // Surface colors
    readonly property color surfacePrimary: purple500
    readonly property color surfacePrimaryHover: purple600
    readonly property color surfacePrimaryActive: purple700
    readonly property color surfaceError: red500
    readonly property color surfaceErrorHover: red600
    readonly property color surfaceErrorActive: red700
}
