import React, { Component, Fragment } from 'react'
import {
  Container,
  Row,
  Col,
  Card,
  CardImg,
  CardText,
  CardBody,
  CardTitle,
  CardSubtitle,
  CardLink,
  Button,
  Badge
} from 'reactstrap'
import CssModules from 'react-css-modules'
import { Link } from 'react-router-dom'
import map from 'lodash/map'
import upperCase from 'lodash/upperCase'
import isEmpty from 'lodash/isEmpty'
import size from 'lodash/size'
import { FiZap } from "react-icons/fi"
import axios from 'axios'

import NavbarMain from '../partials/NavbarMain'
import NavbarBottom from '../partials/NavbarBottom'
import OfferForm from '../partials/OfferForm'
import dummyImg from '../img/dummy-img.png'
import styles from '../css/ListOfPartners.css'

class ListOfPartners extends Component {
  state = {
    showOfferForm: false,
    area: '',
    category: '',
    data: '',
    page: 0,
    totalData: 0,
    elmPartner: []
  }

  componentDidMount() {
    const params = new URLSearchParams(this.props.location.search)

    this.setState({
      area: params.get('area'),
      category: params.get('category'),
      page: 0
    }, () => this.getPartners())
  }

  getPartners() {
    const apiParams = `area=${this.state.area}&category=${this.state.category}&page=${this.state.page}`
    axios({
      method: 'GET',
      url: `/partners/find?${apiParams}`
    })
    .then(response => {
      this.setterDataPartners(response)      
    })
    .catch(error => {
      console.log(error)
    })
  }

  setterDataPartners = (response) => {
    const elmPartner = this.state.elmPartner
    map(response.data.data, (data) => {
      elmPartner.push(
        this.renderPartnersCard(
          data.id,
          data.name,
          data.area,
          data.category_of_trash_id,
          data.address
        )
      )
    })    
    this.setState({
      data: response.data.data,
      totalData: response.data.total,
      elmPartner: elmPartner
    })
  }

  renderOfferForm = () => (
    <OfferForm handleCancelOffer={this.handleCancelOffer} />
  )

  renderPartnersCard(id, name, area, category, address) {
    return <Row className="box" key={id}>
      <Col md={{ size: 4, offset: 4 }}>
        <Card>
          <CardImg top width="100%" src={dummyImg} alt="Card image cap" />
          <CardBody>
            <CardTitle>{name}</CardTitle>
            <CardSubtitle className="trash-category">
              {category.map((val, i) => <Badge color="secondary" key={i}>{upperCase(val)}</Badge>)}
            </CardSubtitle>
            <CardText className="caption">{address}</CardText>
            <CardLink href="#" onMouseDown={this.toggleOfferForm} className="action-menu"><FiZap /><span>Offer</span></CardLink>
          </CardBody>
        </Card>
      </Col>
    </Row>
  }

  renderEmptyPartner = () => (
    <Col md={{ size: 4, offset: 4 }} className="not-found">
      <div className="message">Empty, please search again</div>
      <Link to="/home">
        <Button color="main" size="sm">Search</Button>
      </Link>
    </Col>
  )

  renderPartnerLine = () => (
    <Fragment>
      {this.state.elmPartner}
      {this.state.totalData > size(this.state.elmPartner) &&
        <Row>
          <Col md={{ size: 4, offset: 4 }}>
            <Button outline block size="sm" onMouseDown={this.handleLoadMore}>Load More</Button>
          </Col>
        </Row>}
    </Fragment>
  )

  renderPartners() {
    return <div styleName="ListOfPartners">
      <Container className="basic-container">
        {isEmpty(this.state.data) ? this.renderEmptyPartner(): this.renderPartnerLine()}
      </Container>
    </div>
  }

  toggleOfferForm = () => {
    this.setState({ showOfferForm: !this.state.showOfferForm })
  }

  handleLoadMore = (elmPartner) => {
    this.setState({
      page: this.state.page + 1
    }, () => this.getPartners())
  }

  handleCancelOffer = () => {
    this.setState({ showOfferForm: !this.state.showOfferForm })
  }

  render() {
    return (
      <Fragment>
        <NavbarMain prevRoute={this.props.history.goBack} currentRoute={this.props.location.pathname} />
        {this.state.showOfferForm ? this.renderOfferForm() : this.renderPartners()}
        <NavbarBottom currentRoute={this.props.location.pathname} />
      </Fragment>
    )
  }
}
export default CssModules(ListOfPartners, styles, { allowMultiple: true })