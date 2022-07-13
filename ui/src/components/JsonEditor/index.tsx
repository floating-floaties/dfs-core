import React from "react";
import { Platform, TextInput, StyleSheet } from "react-native";

const EditorLib = require('jsoneditor-react');

export type JsonEditorProps = {
    value: any,
    onChange: (_ev: any) => void,
};

const JsonEditor = ({
    value,
    onChange,
}: JsonEditorProps): JSX.Element => {

    return (
        <EditorLib.JsonEditor
            value={value}
            onChange={onChange}
            // htmlElementProps={}
        />
    )
};


const styles = StyleSheet.create({
    input: {
      height: 40,
      margin: 12,
      borderWidth: 1,
      padding: 10,
    },
  });

export default JsonEditor;