import React, { Component, Fragment } from 'react';
import {
  Container,
  Row,
  Col,
  Button,
  FormGroup,
  Label,
  Input
} from 'reactstrap';
import { Link } from 'react-router-dom';
import CSSModules from 'react-css-modules';
import Select from 'react-select';
import AsyncSelect from 'react-select/lib/Async';
import _ from 'lodash';
import axios from 'axios';

import NavbarBottom from '../partials/NavbarBottom';
import NavbarMain from '../partials/NavbarMain';
import styles from '../css/Home.css';

class Home extends Component {
  constructor(props) {
    super(props)
    this.state = {
      category: null,
      optionsCategory: [],
      area: null,
      optionsArea: [],
    }

    this.handleChangeArea = this.handleChangeArea.bind(this);
    this.handleChangeCategory = this.handleChangeCategory.bind(this);
  }

  componentDidMount() {
    this.getOptionsArea();
    this.getOptionsCategory();
  }

  handleChangeArea(newValue: string) {
    const area = newValue.replace(/\W/g, '');
    this.setState({ area });
    return area;
  }

  handleChangeCategory(newValue: string) {
    const category = newValue.replace(/\W/g, '');
    this.setState({ category });
    return category;
  }

  getOptionsArea() {
    axios({
      method: 'GET',
      url: 'http://localhost:8000/areas'
    })
    .then(response => {
      let DATA = []
      _.map(response.data, (val) => {
        DATA.push({
          "value": val.sub_district_id,
          "label": val.province_name + ", " + val.district_name + ", " + val.sub_district_name
        });        
      });      
      this.setState({ optionsArea: DATA });
    })
    .catch(error => {
      console.log(error);                
    })
    .then(() => {
      
    });
  }

  getOptionsCategory() {
    // axios({
    //   method: 'GET',
    //   url: 'http://localhost:8000/areas'
    // })
    // .then(response => {
    //   let DATA = []
    //   _.map(response.data, (val) => {
    //     DATA.push({
    //       "value": val.sub_district_id,
    //       "label": val.province_name + ", " + val.district_name + ", " + val.sub_district_name
    //     });
    //   });
    //   this.setState({ optionsArea: DATA });
    // })
    // .catch(error => {
    //   console.log(error);
    // })
    // .then(() => {

    // });

    this.setState({
      optionsCategory: [
        { "value": "abcd", "label": "abcd" },
        { "value": "abcd", "label": "abcd" },
        { "value": "abcd", "label": "abcd" },
        { "value": "abcd", "label": "abcd"}
      ]
    })
  }

  loadOptionsArea = (input, callback) => {
    let DATA = this.state.optionsArea;
    if (_.size(input) > 3) {      
      let optionsArea = DATA.filter(i =>
        i.label.toLowerCase().includes(input.toLowerCase())
      );
      setTimeout(() => {
        callback(optionsArea);
      }, 1000);
    }    
  }

  submitToSearch(category, area) {
    this.props.history.push(`/search?category=${this.state.category}&area=${this.state.area}&page=1`)
  }


  render() {
    return (
      <Fragment>
        <NavbarMain />
        <div styleName="Home">
          <Container className="basic-container">
            <Row>
              <Col md={{ size: 6, offset: 3 }}>
                <FormGroup>
                  <Label for="exampleSelect">Category</Label>
                  <Select
                    defaultValue={this.state.optionsCategory[0]}
                    options={this.state.optionsCategory}
                    onInputChange={this.handleChangeCategory}
                  />
                </FormGroup>
                <FormGroup>
                  <Label for="exampleSelect">Area</Label>
                  <AsyncSelect
                    classNamePrefix="select-input"                    
                    cacheOptions
                    loadOptions={this.loadOptionsArea}
                    defaultOptions
                    loadingMessage={() => "minimal 3 character"}
                    noOptionsMessage={() => "area is not found"}
                    onInputChange={this.handleChangeArea}
                  />
                </FormGroup>
                <FormGroup>
                  <Button color="main" size="md" block onClick={this.submitToSearch.bind(this)}>Find</Button>
                </FormGroup>
              </Col>
            </Row>
          </Container>
        </div>
        <NavbarBottom currentRoute={this.props.location.pathname} />          
      </Fragment>
    )
  }
}

export default CSSModules(Home, styles, { allowMultiple: true });
