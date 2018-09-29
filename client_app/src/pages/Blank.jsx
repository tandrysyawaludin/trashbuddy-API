import React, { Component } from 'react'
import * as Cookies from "js-cookie"

import loader from '../img/loader.svg'

class Blank extends Component {
  render() {
    const style = {
      div: {
        background: 'green',
        padding: '10px',
        position: 'absolute',
        top: '40%',
        left: '50%',
        marginTop: '-25px',
        marginLeft: '-25px',
        borderRadius: '30px',
        color: 'white'
      },
      span: {
        marginRight: '10px'
      }
    }
    return (
      <div style={style.div}>
        <span style={style.span}>Loading...</span>
        <img src={loader} />
      </div>      
    )
  }
}

export default Blank
