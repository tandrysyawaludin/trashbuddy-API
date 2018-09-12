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
import { 
  map, 
  size,
  startCase,
  upperCase
} from 'lodash';
import axios from 'axios';

import NavbarBottom from '../partials/NavbarBottom';
import NavbarMain from '../partials/NavbarMain';
import styles from '../css/Home.css';

class Home extends Component {
  constructor(props) {
    super(props)
    this.state = {
      category: "",
      categoryDescription: "",
      optionsCategory: [],
      area: "",
      optionsArea: [],
    }

    this.handleChangeArea = this.handleChangeArea.bind(this);
    this.handleChangeCategory = this.handleChangeCategory.bind(this);
  }

  componentDidMount() {
    this.getOptionsArea();
    this.getOptionsCategory();
  }

  handleChangeArea(area) {
    this.setState({ area: area.value });
  }

  handleChangeCategory(category) {
    this.setState({ category: category.value });
    this.setState({ categoryDescription: category.description });
  }

  getOptionsArea() {
    axios({
      method: 'GET',
      url: 'http://localhost:8000/areas'
    })
    .then(response => {
      let DATA = [];
      let area = '';
      map(response.data, (val) => {
        area = startCase(val.province_name) + ", " + 
              startCase(val.district_name) + ", " + 
              startCase(val.sub_district_name);
        DATA.push({
          "value": val.sub_district_id,
          "label": area
        });        
      });      
      this.setState({ optionsArea: DATA });
    })
    .catch(error => {
      console.log(error);                
    })
  }

  getOptionsCategory() {
    axios({
      method: 'GET',
      url: 'http://localhost:8000/categories_of_trash'
    })
    .then(response => {
      let DATA = []
      map(response.data, (val) => {        
        DATA.push({
          "value": val.name,
          "label": upperCase(val.name),
          "description": val.description
        });
      });
      this.setState({ optionsCategory: DATA });
    })
    .catch(error => {
      console.log(error);
    })
  }

  loadOptionsArea = (input, callback) => {
    let DATA = this.state.optionsArea;
    if (size(input) > 3) {      
      let optionsArea = DATA.filter(i =>
        i.label.toLowerCase().includes(input.toLowerCase())
      );
      setTimeout(() => {
        callback(optionsArea);
      }, 1000);
    }    
  }

  submitToSearch(category, area) {
    this.props.history.push(`/search?area=${this.state.area}&category=${this.state.category}`)
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
                    classNamePrefix="select-input"                                        
                    defaultValue={this.state.optionsCategory[0]}
                    options={this.state.optionsCategory}
                    onChange={this.handleChangeCategory}
                  />
                  {this.state.categoryDescription &&
                    <div className="category-description">{this.state.categoryDescription}</div>}
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
                    onChange={this.handleChangeArea}
                  />
                </FormGroup>
                <FormGroup>
                  <Button color="main" size="md" block
                    disabled={!this.state.category && !this.state.area} 
                    onClick={this.submitToSearch.bind(this)}
                  >Find</Button>
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
