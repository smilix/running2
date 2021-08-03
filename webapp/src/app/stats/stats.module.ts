import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {MonthlyComponent} from './monthly/monthly.component';
import {OverviewComponent} from "./overview/overview.component";
import {MatCardModule} from "@angular/material/card";
import {MatTableModule} from "@angular/material/table";
import {SharedModule} from "../shared/shared.module";
import {RouterModule} from "@angular/router";
import {MatButtonModule} from "@angular/material/button";
import {SimpleStatsTableComponent} from "./simple-stats-table/simple-stats-table.component";
import { DetailsComponent } from './details/details.component';
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatSelectModule} from "@angular/material/select";


@NgModule({
  declarations: [OverviewComponent, MonthlyComponent, SimpleStatsTableComponent, DetailsComponent],
  imports: [
    CommonModule,
    MatCardModule,
    MatTableModule,
    SharedModule,
    RouterModule,
    MatButtonModule,
    MatFormFieldModule,
    MatSelectModule
  ]
})
export class StatsModule {
}
