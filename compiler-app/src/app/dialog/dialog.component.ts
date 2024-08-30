import { Component, Inject } from '@angular/core';
import { Diagnostic } from '../diagnostic';
import { MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'app-dialog',
  templateUrl: './dialog.component.html',
  styleUrls: ['./dialog.component.scss']
})
export class DialogComponent {
  diagnostics: Diagnostic[] = [];

  constructor(@Inject(MAT_DIALOG_DATA) public data: any) {
    this.diagnostics = data.diagnostics;
  }
}
