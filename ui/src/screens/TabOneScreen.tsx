import React from 'react';
import { Platform, StyleSheet, TextInput } from 'react-native';

import EditScreenInfo from '../components/EditScreenInfo';
import { Text, View } from '../components/Themed';
import JsonEditor from '../components/JsonEditor';
import { RootTabScreenProps } from '../../types';
import { useState } from 'react';

export default function TabOneScreen({ navigation }: RootTabScreenProps<'TabOne'>) {
  const [spec, setSpec] = useState<any>({
    system: {
      timezone: 'US/Central',
    },
    context: {
      value: 42,
    },
  });
  const [condition, setCondition] = useState<string>('ctx.value == 42');

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Condition Playground</Text>
      <View style={styles.separator} lightColor="#eee" darkColor="rgba(255,255,255,0.1)" />
      <View style={styles.outputSection}>
        <View style={styles.editor}>
          <JsonEditor
            value={spec}
            onChange={setSpec}
          />
        </View>

        <View style={styles.ioContainer}>
            <TextInput
              style={styles.conditionField}
              value={condition}
              onChangeText={(text) => {
                setCondition(text);
                let origin = window.origin;
                if (origin.includes('localhost')) {
                  origin = 'http://localhost:8080'
                }
                fetch(origin + '/condition', {
                  headers: {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*',
                  },
                  method: 'POST',
                  body: JSON.stringify({
                    spec,
                    condition,
                  })
                })
                .then((value) => value.text())
                .then((value) => {
                  console.log({
                    value,
                  })
                })
              }}
            />
            <Text
              style={styles.outputField}
            >
              Some output here
            </Text>
        </View>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    // alignItems: 'center',
    justifyContent: 'center',
  },
  outputSection: {
    justifyContent: 'space-evenly',
    alignItems: 'center',
    flexDirection: 'row',
  },
  title: {
    textAlign: 'center',
    fontSize: 20,
    fontWeight: 'bold',
  },
  separator: {
    marginVertical: 30,
    height: 1,
    width: '80%',
  },
  editor: {
    flex: 0.5,
  },
  ioContainer: {
    alignItems: 'flex-start',
    justifyContent: 'flex-start'
  },
  conditionField: {
    height: 40,
    margin: 12,
    borderWidth: 1,
    padding: 10,
    // fontFamily: '',

  },

  outputField: {
    paddingTop: 10,
    // fontFamily: 'Consola',

    // backgroundColor: '#cfcfcf',
  },

});
