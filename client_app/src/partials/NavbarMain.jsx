import React, { Component, Fragment } from 'react';
import PropTypes from 'prop-types';
import {
  Collapse,
  Navbar,
  NavbarBrand,
  Nav,
  NavItem
} from 'reactstrap';
import { Link } from 'react-router-dom';
import CssModules from 'react-css-modules';
import { FiArrowLeft, FiX, FiSearch } from "react-icons/fi";

import styles from '../css/NavbarMain.css';
import mainLogo from '../img/logo.png'

class NavbarMain extends Component {
  constructor(props) {
    super(props);

    this.state = {
      searchOnNav: false,
      backOnNav: false,
      closeOnNav: false,
    }
  }

  componentDidMount() {
    if (this.props.currentRoute == "/search") {
      this.setState({
        searchOnNav: true,
        backOnNav: false,
        closeOnNav: false
      })
    }
    else {
      this.setState({
        searchOnNav: false,
        backOnNav: true,
        closeOnNav: false
      })
    }
  }

  render() {    
    return (
      <div styleName="NavbarMain">
        <Navbar fixed="top" light expand="md" className="navbar-default navbar-main">
          {this.state.backOnNav && <FiArrowLeft onMouseDown={this.props.prevRoute} />}
          {/* {this.state.searchOnNav && <Link to="/" styleName="undo-navigation"><FiX /></Link>} */}
          {this.state.searchOnNav && <Link to="/home" className="link-left-icon"><FiSearch /></Link>}
          <NavbarBrand href="#" className="navbar-brand">
            <img src={mainLogo} /><span>Trashbuddy</span>
          </NavbarBrand>
        </Navbar>
      </div>
    )
  }
}

export default CssModules(NavbarMain, styles, { allowMultiple: true });