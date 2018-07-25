import React, { Component } from 'react';
import CssModules from 'react-css-modules';
import styles from '../css/ToggleButton.css';

class ToggleButton extends Component {
  constructor(props) {
    super(props)
  }

  render() {
    return (
      <div styleName="ToggleButton">
        <label styleName="switch">
          <input type="checkbox" onChange={this.props.onChange}/>
          <span styleName="slider"></span>
        </label>
      </div>
    )
  }
}

export default CssModules(ToggleButton, styles, { allowMultiple: true });
